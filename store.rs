use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::image::LoadTexture;

use crate::item::Item;

pub struct Store{    
    item_selected: i32,
    amount_selected: i32,
    price: i32,
    sub_menu: i32,
    number_of_goods: i32,
    item_Rect: Rect,
    money_Rect: Rect,
    amount_Rect: Rect,
    menu_Rect: Rect
}

impl Store {
    pub fn new(number_of_goods: i32) -> Store {
        let item_selected = 0;
        let amount_selected = 1;
        let price = 1;
        let sub_menu = 0;
        let item_Rect = Rect::new(150, 30, 500, 580);
        let money_Rect = Rect::new(660, 570, 470, 40);
        let amount_Rect = Rect::new(660, 510, 470, 40);
        let menu_Rect = Rect::new(150, 30, 500, 580);
        Store {
            item_selected,
            amount_selected,
            price,
            sub_menu,
            number_of_goods,
            item_Rect,
            money_Rect,
            amount_Rect,
            menu_Rect
        }
    }

    pub fn draw(&self,wincan: &mut WindowCanvas){ 
        let item_Rect = Rect::new(150, 30, 500, 580);
    


        //draw menu canvas
        wincan.set_draw_color(Color::RGBA(159,82,30,255));
        wincan.fill_rect(Rect::new(140, 20, 1000, 600));

        // draw menu features 
        wincan.set_draw_color(Color::RGBA(244,182,110,255));
        // items
        wincan.fill_rect(Rect::new(150, 30, 500, 580));
        // image 
        wincan.fill_rect(Rect::new(660, 30, 470, 470));
        // amount and price
        wincan.fill_rect(Rect::new(660, 510, 470, 100));

        // selection 
        wincan.set_draw_color(Color::RGBA(200,0,0,255));
        wincan.fill_rect(Rect::new(150, 30 + self.item_selected*50, 500, 50));

        // submenu
        wincan.set_draw_color(Color::RGBA(0,0,0,40));
        wincan.fill_rect(self.menu_Rect);
        Store::price_draw(wincan, 6, 665, 578, self.price);
        Store::price_draw(wincan, 6, 665, 518, self.amount_selected);


        let texture_creator = wincan.texture_creator();
        let Lable_Texture = texture_creator.load_texture("src/images/MoneyLabels.png").unwrap();
        wincan.copy(&Lable_Texture, Rect::new(0, 0, 16, 5), Rect::new(850,578,80,25));
        wincan.copy(&Lable_Texture, Rect::new(18, 0, 20, 5), Rect::new(850,518,100,25));

        let img_Texture = texture_creator.load_texture("src/images/ItemImagePlaceHolder.png").unwrap();
        wincan.copy(&img_Texture, None, Rect::new(665, 35, 460, 460));

        }

    pub fn navigate(&mut self, increment :i32){
        if self.sub_menu == 0 {
            if increment == -1 && self.item_selected != 0{
                self.item_selected = self.item_selected + increment;
                self.price = 1;
                self.amount_selected = 1;
            }
            if increment == 1 && self.item_selected != 10{
                self.item_selected = self.item_selected + increment;
                self.price = 1;
                self.amount_selected = 1;
            }    
        }

        if self.sub_menu == 2 {
            if increment == -1 && self.price != 99999{
                self.price = self.price - increment;

            }
            if increment == 1 && self.price != 1{
                self.price = self.price - increment;
            }    
        }

        if self.sub_menu == 1 {
            if increment == -1 && self.amount_selected != self.number_of_goods{
                self.price = self.price + (self.price/self.amount_selected);
                self.amount_selected = self.amount_selected - increment;

            }
            if increment == 1 && self.amount_selected != 1{
                self.amount_selected = self.amount_selected - increment;
                self.price = self.price - (self.price/self.amount_selected);
                
            }    
        }


    }

    pub fn cycle(&mut self, increment :i32){
        if increment == -1 && self.sub_menu != 0{
            self.sub_menu = self.sub_menu + increment;
        }
        if increment == 1 && self.sub_menu != 2{
            self.sub_menu = self.sub_menu + increment;
        }
        if self.sub_menu == 0 {
            self.menu_Rect = self.item_Rect;
        }
        if self.sub_menu == 1 {
            self.menu_Rect = self.amount_Rect;
        }
        if self.sub_menu == 2 {
            self.menu_Rect = self.money_Rect;
        }   
        
    }

    pub fn price_draw(wincan: &mut WindowCanvas, mut steps: i32, initx: i32, inity: i32, value: i32){
        let texture_creator = wincan.texture_creator();
        let values_texture = texture_creator.load_texture("src/images/MoneySpriteSheet.png").unwrap();
        let initsteps = steps;
        let mut disp_price = value;
        let mut tailing_zero = false;
        while steps >= 0 {
            let temp = i32::pow(10, steps as u32);
            let mut modulo = (disp_price - (disp_price % temp)) /temp;
            disp_price = disp_price - (modulo*temp);
            if tailing_zero && modulo == 0 {
                modulo = 10;
            }

            if modulo != 0 {
                tailing_zero = true;
            }

            wincan.copy(&values_texture, Rect::new(5*modulo, 0, 5, 5), Rect::new((initx + (initsteps-steps)*25) as i32, inity, 25, 25));
            
            steps = steps - 1;
            
        }
    }


}

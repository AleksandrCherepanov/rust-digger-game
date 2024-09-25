use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas};
use crate::animation::Animation;
use crate::resources::background::draw_background;
use crate::resources::text;
use crate::sprites::{Animated, Drawable, Sprite};
use crate::sprites::bag::Bag;
use crate::sprites::bonus::Bonus;
use crate::sprites::digger::Digger;
use crate::sprites::emerald::Emerald;
use crate::sprites::hobbin::Hobbin;
use crate::sprites::nobbin::Nobbin;

pub struct Screen {
    canvas: WindowCanvas,
}

impl Screen {
    pub fn new(sdl: &Sdl) -> Screen {
        let video = sdl.video().unwrap();
        let window = video.window("D I G G E R", 640, 400).build().unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Screen {
            canvas
        }
    }

    pub fn initial(&mut self) {
        self.show_background();
        self.show_game_name();
        self.show_players();
    }

    fn show_game_name(&mut self) {
        text::print(&mut self.canvas, "D I G G E R", 100, 0, 3);
    }

    fn show_players(&mut self) {
        text::print(&mut self.canvas, "ONE", 220, 25, 3);
        text::print(&mut self.canvas, " PLAYER ", 192, 39, 3);
    }

    fn show_background(&mut self) {
        draw_background(&mut self.canvas);
    }

    pub fn play_initial_nobbin_animation(&mut self, a: &mut Animation, f: usize, x: i32) -> i32 {
        let mut x = x;

        if f == 50 {
            self.draw_sprite::<Nobbin>(0, 292, 63);
        }
        if f > 50 && f <= 77 {
            x -= 4;
            self.draw_animation::<Nobbin>(a, Animation::left(), x, 63);
        }
        if f > 77 {
            self.draw_animation::<Nobbin>(a, Animation::right(), 184, 63)
        }
        if f == 84 {
            self.draw_text("NOBBIN", 216, 64, 2);
        }

        x
    }

    pub fn play_initial_hobbin_animation(&mut self, a: &mut Animation, f: usize, x: i32) -> i32 {
        let mut x = x;

        if f == 90 {
            self.draw_sprite::<Hobbin>(0, 292, 82);
            x = 292;
        }
        if f > 90 && f <= 117 {
            x -= 4;
            self.draw_animation::<Hobbin>(a, Animation::left(), x, 82);
        }
        if f > 117 {
            self.draw_animation::<Hobbin>(a, Animation::right(), 184, 82);
        }
        if f == 123 {
            self.draw_text("HOBBIN", 216, 83, 2);
        }

        x
    }

    pub fn play_initial_digger_animation(&mut self, a: &mut Animation, f: usize, x: i32) -> i32 {
        let mut x = x;

        if f == 130 {
            self.draw_sprite::<Digger>(9, 292, 101);
            x = 292;
        }
        if f > 130 && f <= 157 {
            x -= 4;
            self.draw_animation::<Digger>(a, Animation::left(), x, 101);
        }
        if f > 157 {
            self.draw_animation::<Digger>(a, Animation::right(), 184, 101);
        }
        if f == 163 {
            self.draw_text("DIGGER", 216, 102, 2);
        }

        x
    }

    pub fn show_initial_items(&mut self, f: usize) {
        if f == 178 {
            self.draw_sprite::<Bag>(0, 184, 120);
        }
        if f == 183 {
            self.draw_text("GOLD", 216, 121, 2);
        }
        if f == 198 {
            self.draw_sprite::<Emerald>(0, 184, 141);
        }
        if f == 203 {
            self.draw_text("EMERALD", 216, 140, 2);
        }
        if f == 218 {
            self.draw_sprite::<Bonus>(0, 184, 158);
        }
        if f == 223 {
            self.draw_text("BONUS", 216, 159, 2);
        }
    }

    pub fn clean_initial_items(&mut self, f: usize) {
        if f == 0 {
            let mut i = 54;
            while i < 174 {
                self.draw_text("            ", 164, i, 0);
                i += 12;
            }
        }
    }

    pub fn clean(&mut self) {
        self.canvas.clear();
    }

    pub fn draw_sprite<T: Drawable>(&mut self, id: usize, x: i32, y: i32) {
        Sprite::<T>::new(id).draw(&mut self.canvas, x, y);
    }

    pub fn draw_animation<T: Drawable + Animated>(&mut self, animation: &mut Animation, direction: i32, x: i32, y: i32) {
        animation.play::<T>(&mut self.canvas, direction, x, y);
    }

    pub fn draw_text(&mut self, text: &str, x: i32, y: i32, flag: i32) {
        text::print(&mut self.canvas, text, x, y, flag);
    }
    pub fn draw_number(&mut self, num: usize, x: i32, y: i32, width: i32, flag: i32) {
        text::print_number(&mut self.canvas, num, x, y, width, flag);
    }

    pub fn render(&mut self) {
        self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        self.canvas.present();
    }
}

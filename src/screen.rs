use std::thread::sleep;
use std::time::Duration;
use sdl2::{EventPump, Sdl};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas};
use crate::resources::background::draw_background;
use crate::resources::palette::get_color;
use crate::resources::text;
use crate::scores::Scores;
use crate::settings::Settings;
use crate::sprites::back::Back;
use crate::sprites::bag::Bag;
use crate::sprites::bonus::Bonus;
use crate::sprites::digger::Digger;
use crate::sprites::emerald::Emerald;
use crate::sprites::explosive::Explosive;
use crate::sprites::fire::Fire;
use crate::sprites::gold::Gold;
use crate::sprites::grave::Grave;
use crate::sprites::hobbin::Hobbin;
use crate::sprites::life::Life;
use crate::sprites::nobbin::Nobbin;
use crate::sprites::{Drawable, Sprite};

pub struct Screen {
    canvas: WindowCanvas,
}

impl Screen {
    pub fn new(sdl: &Sdl) -> Screen {
        let video = sdl.video().unwrap();
        let window = video.window("D I G G E R", 640, 400).build().unwrap();
        let canvas = window.into_canvas().build().unwrap();
        sdl.mouse().show_cursor(false);

        Screen {
            canvas
        }
    }

    pub fn show_game_name(&mut self) {
        text::print(&mut self.canvas, "D I G G E R", 100, 0, 3);
    }

    pub fn show_players(&mut self, settings: &Settings) {
        if settings.get_diggers() == 2 {
            if settings.is_gauntlet_mode() {
                text::print(&mut self.canvas, "TWO PLAYER", 180, 25, 3);
                text::print(&mut self.canvas, "GAUNTLET", 192, 39, 3);
                return;
            }

            text::print(&mut self.canvas, "TWO PLAYER", 180, 25, 3);
            text::print(&mut self.canvas, "SIMULTANEOUS", 170, 39, 3);
            return;
        }

        if settings.is_gauntlet_mode() {
            text::print(&mut self.canvas, "GAUNTLET", 192, 25, 3);
            text::print(&mut self.canvas, "MODE", 216, 39, 3);
            return;
        }

        if settings.get_players() == 1 {
            text::print(&mut self.canvas, "ONE", 220, 25, 3);
            text::print(&mut self.canvas, " PLAYER ", 192, 39, 3);
            return;
        }

        text::print(&mut self.canvas, "TWO", 220, 25, 3);
        text::print(&mut self.canvas, " PLAYERS", 184, 39, 3);
    }

    pub fn show_scores(&mut self, scores: &Scores) {
        self.draw_text("HIGH SCORES", 16, 25, 3);
        let mut color = 2;
        for i in 1..scores.scores_init.len() {
            let mut line: String = String::from(&scores.scores_init[i]);
            let score: String = format!("{:6}", scores.scores_high[i + 1]);
            line = String::from(line) + "  " + &score;
            self.draw_text(&line, 16, (31 + 13*i) as i32, color);
            color = 1;
        }
    }

    pub fn show_background(&mut self) {
        draw_background(&mut self.canvas);
    }

    pub fn show_nobbin(&mut self) {
        let mut xx = 10;
        let yy = 10;
        for i in 0..4 {
            Sprite::<Nobbin>::new(i).draw(&mut self.canvas, xx, yy);
            xx += 32 + 10;
        }

    }

    pub fn show_digger(&mut self) {
        let mut xx = 10;
        let yy = 50;
        for i in 1..14 {
            Sprite::<Digger>::new(i).draw(&mut self.canvas, xx, yy);
            xx += 32 + 10;
        }
    }

    pub fn show_grave(&mut self) {
        let mut xx = 10;
        let yy = 90;
        for i in 0..5 {
            Sprite::<Grave>::new(i).draw(&mut self.canvas, xx, yy);
            xx += 32 + 10;
        }
    }

    pub fn show_bags(&mut self) {
        let mut xx = 220;
        let yy = 90;
        for i in 0..4 {
            Sprite::<Bag>::new(i).draw(&mut self.canvas, xx, yy);
            xx += 32 + 10;
        }
    }

    pub fn show_golds(&mut self) {
        let mut xx = 388;
        let yy = 90;
        for i in 0..3 {
            Sprite::<Gold>::new(i).draw(&mut self.canvas, xx, yy);
            xx += 32 + 10;
        }

        Sprite::<Emerald>::new(0).draw(&mut self.canvas, xx, yy);
    }

    pub fn show_hobbins(&mut self) {
        let mut xx = 178;
        let yy = 10;
        for i in 0..8 {
            Sprite::<Hobbin>::new(i).draw(&mut self.canvas, xx, yy);
            xx += 32 + 10;
        }
    }

    pub fn show_bonuses(&mut self) {
        let mut xx = 514;
        let yy = 10;
        for i in 0..1 {
            Sprite::<Bonus>::new(i).draw(&mut self.canvas, xx, yy);
            xx += 32 + 10;
        }
    }

    pub fn show_fire(&mut self) {
        let mut xx = 10;
        let yy = 130;
        for i in 0..3 {
            Sprite::<Fire>::new(i).draw(&mut self.canvas, xx, yy);
            xx += 32 + 10;
        }

        for i in 0..3 {
            Sprite::<Explosive>::new(i).draw(&mut self.canvas, xx, yy);
            xx += 32 + 10;
        }
    }

    pub fn show_backs(&mut self) {
        let mut xx = 10;
        let yy = 170;
        for i in 0..8 {
            Sprite::<Back>::new(i).draw(&mut self.canvas, xx, yy);
            xx += 40 + 10;
        }

        for i in 0..3 {
            Sprite::<Life>::new(i).draw(&mut self.canvas, xx, yy);
            xx += 32 + 10;
        }
    }

    pub fn show_animated(&mut self) {
        let mut xx = 288;
        let yy = 63;
        let mut n = 0;
        for i in 0..28 {
            if n > 3 {
                n = 0;
            }
            self.canvas.set_draw_color(Color::BLACK);
            self.canvas.fill_rect(Rect::new((xx + 4) * 2, yy, 62, 60)).unwrap();
            Sprite::<Nobbin>::new(n).draw(&mut self.canvas, (xx + 4) * 2, yy);
            xx -= 4;
            n += 2;
            self.render();
            sleep(Duration::from_millis(120));
        }
    }

    pub fn clean(&mut self) {
        self.canvas.clear();
    }

    pub fn draw_sprite<T: Drawable>(&mut self, id: usize, x: i32, y: i32) {
        Sprite::<T>::new(id).draw(&mut self.canvas, x * 2, y * 2);
    }

    pub fn draw_text(&mut self, text: &str, x: i32, y: i32, flag: i32) {
        text::print(&mut self.canvas, text, x, y, flag);
    }

    pub fn render(&mut self) {
        self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        self.canvas.present();
    }
}

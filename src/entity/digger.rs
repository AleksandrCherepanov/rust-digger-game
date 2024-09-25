use crate::animation::Animation;
use crate::screen::Screen;
use crate::sprites::life::Life;
use crate::sprites::digger::Digger as DiggerSprite;

#[derive(Default)]
pub struct Digger {
    pub x: i32,
    pub y: i32,
    pub h: i32,
    pub v: i32,
    pub rx: i32,
    pub ry: i32,
    pub mdir: i32,
    pub dir: i32,
    pub bagtime: i32,
    pub rechargetime: i32,
    pub fx: i32,
    pub fy: i32,
    pub fdir: i32,
    pub expsn: i32,
    pub deathstage: i32,
    pub deathbag: i32,
    pub deathani: i32,
    pub deathtime: i32,
    pub emocttime: i32,
    pub emn: i32,
    pub msc: i32,
    pub lives: i32,
    pub ivt: i32,
    pub notfiring: bool,
    pub alive: bool,
    pub firepressed: bool,
    pub dead: bool,
    pub levdone: bool,
    pub invin: bool
}

impl Digger {
    pub fn add_lives(&mut self) {
        if self.lives < 5 {
            self.lives += 1;
        }
    }

    pub fn draw_lives(&mut self, screen: &mut Screen) {
        let mut n = self.lives - 1;
        screen.draw_text("     ", 96, 0, 2);
        if n > 4 {
            screen.draw_sprite::<Life>(0, 80, 0);
            screen.draw_text(&format!("X{}", n), 100, 0, 2);
        } else {
            for l in 1..5 {
                let mut sprite_id = 0;
                if n <= 0 {
                    sprite_id = 2;
                }
                screen.draw_sprite::<Life>(sprite_id, l * 20 + 60, 0);
                n -= 1;
            }
        }
    }

    pub fn animate(&mut self, screen: &mut Screen, a: &mut Animation, direction: i32) {
        screen.draw_animation::<DiggerSprite>(a, direction, self.x, self.y);
    }
}
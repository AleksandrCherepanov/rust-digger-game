use crate::animation::Animation;
use crate::resources::level::{LEVELS, SIZE, WIDTH, HEIGHT, is_spawn, is_vertical_path, is_horizontal_path, is_emerald, is_bag};
use crate::sprites::emerald::Emerald;
use crate::entity::bag::Bag as EntityBag;
use crate::entity::digger::Digger as EntityDigger;
use crate::screen::Screen;
use crate::sprites::back::Back;
use crate::sprites::bag::Bag;
use crate::sprites::border::{BorderBottom, BorderLeft, BorderRight, BorderTop};

pub struct Level {
    level: usize,
    field: [i16; SIZE],
    emeralds: [isize; SIZE],
    bags: Vec<EntityBag>,
    pub digger: EntityDigger,
}

impl Level {
    pub fn new(level: usize) -> Level {
        Level {
            field: [0; SIZE],
            emeralds: [0; SIZE],
            bags: Vec::new(),
            level,
            digger: EntityDigger::default(),
        }
    }

    fn get_level_safe(&self) -> usize {
        let mut level = self.level;
        if level > 8 {
            level = (self.level & 3) + 5
        }

        level
    }

    fn get_level_tile(&self, x: usize, y: usize) -> u8 {
        let level = self.get_level_safe();
        LEVELS[level - 1][y][x]
    }

    pub fn init_field(&mut self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                self.field[y * WIDTH + x] = -1;
                let c = self.get_level_tile(x, y);
                if is_spawn(c) || is_vertical_path(c) {
                    self.field[y * WIDTH + x] &= 0xd03fu16 as i16;
                }
                if is_spawn(c) || is_horizontal_path(c) {
                    self.field[y * WIDTH + x] &= 0xdfe0u16 as i16;
                }
            }
        }
    }

    pub fn init_emeralds(&mut self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let c = self.get_level_tile(x, y);
                if is_emerald(c) {
                    self.emeralds[y * WIDTH + x] |= 1;
                } else {
                    self.emeralds[y * WIDTH + x] &= 0;
                }
            }
        }
    }

    pub fn init_bags(&mut self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let c = self.get_level_tile(x, y);
                if is_bag(c) {
                    self.bags.push(EntityBag::new(x as i32, y as i32))
                }
            }
        }
    }

    pub fn init_diggers(&mut self) {
        self.digger.v = 9;
        self.digger.mdir = 4;
        self.digger.h = 7;
        self.digger.x = self.digger.h * 20 + 12;
        self.digger.dir = Animation::right();
        self.digger.rx = 0;
        self.digger.ry = 0;
        self.digger.bagtime = 0;
        self.digger.alive = true;
        self.digger.dead = false;
        self.digger.invin = false;
        self.digger.ivt = 0;
        self.digger.deathstage = 1;
        self.digger.y = self.digger.v * 18 + 18;
        self.digger.notfiring = true;
        self.digger.emocttime = 0;
        self.digger.firepressed = false;
        self.digger.expsn = 0;
        self.digger.rechargetime = 0;
        self.digger.emn = 0;
        self.digger.msc = 1;
        self.digger.lives = 3;
    }

    pub fn draw(&mut self, screen: &mut Screen) {
        self.draw_back(screen);
        self.draw_path(screen);
        self.draw_emeralds(screen);
        self.draw_bags(screen);
    }

    fn draw_emeralds(&mut self, screen: &mut Screen) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if self.emeralds[y * WIDTH + x] == 1 {
                    screen.draw_sprite::<Emerald>(0, (x * 20 + 12) as i32, (y * 18 + 21) as i32);
                }
            }
        }
    }

    fn draw_bags(&mut self, screen: &mut Screen) {
        for b in &self.bags {
            screen.draw_sprite::<Bag>(0, b.x, b.y);
        }
    }

    fn draw_back(&mut self, screen: &mut Screen) {
        let level = self.get_level_safe();

        let mut y = 14;
        while y < 200 {
            let mut x = 0;
            while x < 320 {
                screen.draw_sprite::<Back>(level - 1, x, y);
                x += 20;
            }
            y += 4;
        }
    }

    fn draw_path(&mut self, screen: &mut Screen) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if self.field[y * WIDTH + x] & 0x2000 == 0 {
                    let xx = x * 20 + 12;
                    let yy = y * 18 + 18;

                    if self.field[y * WIDTH + x] & 0xfc0 != 0xfc0 {
                        self.field[y * WIDTH + x] &= 0xd03fu16 as i16;
                        let mut i = 15;
                        while i >= 3 {
                            let rx = xx as i32 - 4;
                            let ry = yy as i32 - i + 15;
                            screen.draw_sprite::<BorderBottom>(2, rx, ry);
                            i -= 3;
                        }
                        let rx = xx as i32 - 4;
                        let ry = yy as i32 - 6 + 3;
                        screen.draw_sprite::<BorderTop>(0, rx, ry);
                    }
                    if self.field[y * WIDTH + x] & 0x1f != 0x1f {
                        self.field[y * WIDTH + x] &= 0xdfe0u16 as i16;
                        let mut i = 16;
                        while i >= 4 {
                            let rx = xx as i32 - i + 16;
                            let ry = yy as i32 - 1;
                            screen.draw_sprite::<BorderRight>(1, rx, ry);
                            i -= 4;
                        }
                        let rx = xx as i32 + 4 - 8;
                        let ry = yy as i32 - 1;
                        screen.draw_sprite::<BorderLeft>(3, rx, ry);
                    }
                    if x < 14 && (self.field[y * WIDTH + x + 1] & 0xfdf != 0xfdf) {
                        let rx = xx as i32 + 16;
                        let ry = yy as i32 - 1;
                        screen.draw_sprite::<BorderRight>(1, rx, ry);
                    }

                    if y < 9 && (self.field[(y + 1) * WIDTH + x] & 0xfdf != 0xfdf) {
                        let rx = xx as i32 - 4;
                        let ry = yy as i32 + 15;
                        screen.draw_sprite::<BorderBottom>(2, rx, ry);
                    }
                }
            }
        }
    }
}

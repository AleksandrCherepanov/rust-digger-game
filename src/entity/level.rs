use sdl2::render::WindowCanvas;
use crate::resources::level::{LEVELS, SIZE, WIDTH, HEIGHT, is_spawn, is_vertical_path, is_horizontal_path, is_emerald, is_bag};
use crate::sprites::emerald::Emerald;
use crate::sprites::Sprite;
use crate::entity::bag::Bag as EntityBag;
use crate::entity::digger::Digger as EntityDigger;
use crate::sprites::back::Back;
use crate::sprites::bag::Bag;
use crate::sprites::border::{BorderBottom, BorderLeft, BorderRight, BorderTop};

pub struct Level {
    level: usize,
    field: [i16; SIZE],
    emeralds: [isize; SIZE],
    bags: Vec<EntityBag>,
    diggers: Vec<EntityDigger>
}

impl Level {
    pub fn new(level: usize) -> Level {
        Level {
            field: [0; SIZE],
            emeralds: [0; SIZE],
            bags: Vec::new(),
            level,
            diggers: vec![EntityDigger::default()]
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

    }

    pub fn draw_emeralds(&mut self, canvas: &mut WindowCanvas) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if self.emeralds[y * WIDTH + x] == 1 {
                    Sprite::<Emerald>::new(0).draw(canvas, (x * 20 + 12) as i32, (y * 18 + 21) as i32)
                }
            }
        }
    }

    pub fn draw_bags(&mut self, canvas: &mut WindowCanvas) {
        for b in &self.bags {
            Sprite::<Bag>::new(0).draw(canvas, b.x, b.y)
        }
    }

    pub fn draw_back(&mut self, canvas: &mut WindowCanvas) {
        let level = self.get_level_safe();

        let mut y = 14;
        while y < 200 {
            let mut x = 0;
            while x < 320 {
                Sprite::<Back>::new(level - 1).draw(canvas, x, y);
                x += 20;
            }
            y += 4;
        }
    }

    pub fn draw_path(&mut self, canvas: &mut WindowCanvas) {
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
                            Sprite::<BorderBottom>::new(2).draw(canvas, rx, ry);
                            i -= 3;
                        }
                        let rx = xx as i32 - 4;
                        let ry = yy as i32 - 6 + 3;
                        Sprite::<BorderTop>::new(0).draw(canvas, rx, ry);
                    }
                    if self.field[y * WIDTH + x] & 0x1f != 0x1f {
                        self.field[y * WIDTH + x] &= 0xdfe0u16 as i16;
                        let mut i = 16;
                        while i >= 4 {
                            let rx = xx as i32 - i + 16;
                            let ry = yy as i32 - 1;
                            Sprite::<BorderRight>::new(1).draw(canvas, rx, ry);
                            i -= 4;
                        }
                        let rx = xx as i32 + 4 - 8;
                        let ry = yy as i32 - 1;
                        Sprite::<BorderLeft>::new(3).draw(canvas, rx, ry);
                    }
                    if x < 14 && (self.field[y * WIDTH + x + 1] & 0xfdf != 0xfdf) {
                        let rx = xx as i32 + 16;
                        let ry = yy as i32 - 1;
                        Sprite::<BorderRight>::new(1).draw(canvas, rx, ry);
                    }

                    if y < 9 && (self.field[(y + 1) * WIDTH + x] & 0xfdf != 0xfdf) {
                        let rx = xx as i32 - 4;
                        let ry = yy as i32 + 15;
                        Sprite::<BorderBottom>::new(2).draw(canvas, rx, ry);
                    }
                }
            }
        }
    }
}

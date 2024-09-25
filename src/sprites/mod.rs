use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use crate::resources::palette::get_color;

pub mod nobbin;
pub mod digger;
pub mod grave;
pub mod bag;
pub mod gold;
pub mod hobbin;
pub mod bonus;
pub mod fire;
pub mod explosive;
pub mod back;
pub mod emerald;
pub mod life;
pub mod border;

const WIDTH_RATIO: usize = 2;
const HEIGHT_RATIO: usize = 2;

pub trait Drawable {
    fn new(id: usize) -> Self;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn size(&self) -> usize;
    fn color(&self, idx: usize) -> usize;
}

pub trait Animated {
    fn left() -> (i32, i32);
    fn right() -> (i32, i32);
}

pub struct Sprite<T: Drawable> {
    sprite: T,
}

impl<T: Drawable> Sprite<T> {
    pub fn new(id: usize) -> Sprite<T> {
        Sprite {
            sprite: T::new(id)
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, x: i32, y: i32) {
        let initial_x = x * WIDTH_RATIO as i32;
        let mut xx = initial_x;
        let mut yy = y * HEIGHT_RATIO as i32;

        for i in 0..self.sprite.size() {
            let color = self.sprite.color(i);
            if color != 0xff {
                canvas.set_draw_color(get_color(color));
                canvas.draw_point(Point::new(xx, yy)).unwrap();
            }

            if (i + 1) % self.sprite.width() == 0 {
                xx = initial_x;
                yy += 1;
            }
            xx += 1;
        }
    }

    pub fn clean(&self, canvas: &mut WindowCanvas, x: i32, y: i32) {
        canvas.set_draw_color(Color::BLACK);
        let real_width = self.sprite.width() + 1;
        let real_height = self.sprite.height();
        let real_x = x * WIDTH_RATIO as i32;
        let real_y = y * HEIGHT_RATIO as i32;
        let rect = Rect::new(real_x, real_y, real_width as u32, real_height as u32);
        canvas.fill_rect(rect).unwrap()
    }
}

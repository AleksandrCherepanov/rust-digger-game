use sdl2::rect::Point;
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

pub trait Drawable {
    fn new(id: usize) -> Self;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn size(&self) -> usize;
    fn color(&self, idx: usize) -> usize;
}

pub struct Sprite<T: Drawable> {
    sprite: T
}

impl<T: Drawable> Sprite<T> {
    pub fn new (id: usize) -> Sprite<T> {
        Sprite {
            sprite: T::new(id)
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, x: i32, y: i32) {
        let mut xx = x;
        let mut yy = y;

        for i in 0..self.sprite.size() {
            let color = self.sprite.color(i);
            if  color != 0xff {
                canvas.set_draw_color(get_color(color));
                canvas.draw_point(Point::new(xx, yy)).unwrap();
            }

            if (i + 1) % self.sprite.width() == 0 {
                xx = x;
                yy += 1;
            }
            xx += 1;
        }
    }
}
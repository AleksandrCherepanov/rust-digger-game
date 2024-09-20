use sdl2::render::WindowCanvas;
use crate::sprites::{Animated, Drawable, Sprite};

pub struct Animation {
    current_sprite: i32,
    prev_x: i32,
    prev_y: i32,
}

impl Animation {
    pub fn new(prev_x: i32, prev_y: i32) -> Animation {
        Animation {
            current_sprite: -1,
            prev_x,
            prev_y,
        }
    }

    pub fn left() -> i32 {
        -1
    }

    pub fn right() -> i32 {
        1
    }

    pub fn play<T: Drawable + Animated>(
        &mut self,
        canvas: &mut WindowCanvas,
        direction: i32,
        x: i32,
        y: i32,
    ) {
        let mut first_sprite: i32;
        let mut last_sprite: i32;

        if direction == Self::left() {
            first_sprite = T::left().0;
            last_sprite = T::left().1;
        } else {
            first_sprite = T::right().0;
            last_sprite = T::right().1;
        }

        if self.current_sprite < first_sprite {
            if direction == Self::left() {
                self.current_sprite = last_sprite;
            } else {
                self.current_sprite = first_sprite;
            }
        } else {
            self.current_sprite += 1 * direction;
            if direction == Self::left() && self.current_sprite < first_sprite {
                self.current_sprite = last_sprite;
            }
            if direction == Self::right() && self.current_sprite > last_sprite {
                self.current_sprite = first_sprite;
            }
        }

        let sprite = Sprite::<T>::new(self.current_sprite as usize);
        sprite.clean(canvas, self.prev_x, self.prev_y);

        sprite.draw(canvas, x, y);
        self.prev_x = x;
        self.prev_y = y;
    }
}

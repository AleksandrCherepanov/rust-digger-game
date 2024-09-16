use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use crate::resources::font::{FONT, LETTER_WIDTH};
use crate::resources::palette::get_color;

pub fn print(canvas: &mut WindowCanvas, text: &str, x: i32, y: i32, flag: i32) {
    let mut x = x * 2;
    for c in text.chars() {
        if !c.is_ascii() {
            continue;
        }

        let letter = FONT[(c as usize) - 32];
        if letter[0] < 0 {
            continue;
        }

        print_letter(canvas, &letter, x, y, flag);
        x += LETTER_WIDTH as i32;
    }
}

fn print_letter(canvas: &mut WindowCanvas, letter: &[i8; 24 * 24], x: i32, y: i32, flag: i32) {
    let mut xx = x;
    let mut yy = y * 2;

    for i in 0..letter.len() {
        let color = get_color(calc_color(letter[i], flag));
        canvas.set_draw_color(color);

        canvas.draw_point(Point::new(xx, yy)).unwrap();
        if (i + 1) % LETTER_WIDTH == 0 {
            xx = x;
            yy += 1;
        }
        xx += 1;
    }
}

fn calc_color(code: i8, flag: i32) -> usize {
    if code == 10 {
        if flag == 2 {
            return 12;
        }
    }
    if code == 12 {
        if flag == 1 {
            return 2;
        }

        if flag == 2 {
            return 4;
        }

        if flag == 3 {
            return 6;
        }
    }

    code as usize
}

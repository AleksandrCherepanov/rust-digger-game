use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use crate::resources::font::{FONT, WIDTH};
use crate::resources::palette::get_color;

pub fn print_number(canvas: &mut WindowCanvas, num: usize, x: i32, y: i32, width: i32, flag: i32) {
    let mut w = width;
    let mut n = num;
    let mut text = String::new();

    while w > 0 {
        let d = n % 10;
        if w > 1 || d > 0 {
            text.push(char::from_digit(d as u32, 10).unwrap())
        }
        n /= 10;
        w -= 1;
    }

    print(canvas, &text, x, y, flag);
}

pub fn print(canvas: &mut WindowCanvas, text: &str, x: i32, y: i32, flag: i32) {
    let mut x = x;
    for c in text.chars() {
        if !c.is_ascii() {
            continue;
        }

        let letter = FONT[(c as usize) - 32];
        if letter[0] < 0 {
            continue;
        }

        print_letter(canvas, &letter, x, y, flag);
        x += WIDTH as i32 / 2;
    }
}

fn print_letter(canvas: &mut WindowCanvas, letter: &[i8; 24 * 24], x: i32, y: i32, flag: i32) {
    let initial_x = x * 2;
    let mut xx = initial_x;
    let mut yy = y * 2;

    for i in 0..letter.len() {
        let color = get_color(calc_color(letter[i], flag));
        canvas.set_draw_color(color);

        canvas.draw_point(Point::new(xx, yy)).unwrap();
        if (i + 1) % WIDTH == 0 {
            xx = initial_x;
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

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

const PALETTE: [Color; 16] = [
    Color::RGBA(0, 0, 0, 0),
    Color::RGBA(0, 0, 128, 0),
    Color::RGBA(0, 128, 0, 0),
    Color::RGBA(0, 128, 128, 0),
    Color::RGBA(128, 0, 0, 0),
    Color::RGBA(128, 0, 128, 0),
    Color::RGBA(128, 64, 0, 0),
    Color::RGBA(128, 128, 128, 0),
    Color::RGBA(64, 64, 64, 0),
    Color::RGBA(0, 0, 255, 0),
    Color::RGBA(0, 255, 0, 0),
    Color::RGBA(0, 255, 255, 0),
    Color::RGBA(255, 0, 0, 0),
    Color::RGBA(255, 0, 255, 0),
    Color::RGBA(255, 255, 0, 0),
    Color::RGBA(255, 255, 255, 0),
];

const PALETTE_INTENSE: [Color; 16] = [
    Color::RGBA(0, 0, 0, 0),
    Color::RGBA(0, 0, 255, 0),
    Color::RGBA(0, 255, 0, 0),
    Color::RGBA(0, 255, 255, 0),
    Color::RGBA(255, 0, 0, 0),
    Color::RGBA(255, 0, 255, 0),
    Color::RGBA(255, 128, 0, 0),
    Color::RGBA(196, 196, 196, 0),
    Color::RGBA(128, 128, 128, 0),
    Color::RGBA(128, 128, 255, 0),
    Color::RGBA(128, 255, 128, 0),
    Color::RGBA(128, 255, 255, 0),
    Color::RGBA(255, 128, 128, 0),
    Color::RGBA(255, 128, 255, 0),
    Color::RGBA(255, 255, 128, 0),
    Color::RGBA(255, 255, 255, 0),
];

const PALETTE2: [Color; 16] = [
    Color::RGBA(0, 0, 0, 0),
    Color::RGBA(0, 128, 0, 0),
    Color::RGBA(128, 0, 0, 0),
    Color::RGBA(128, 64, 0, 0),
    Color::RGBA(0, 0, 128, 0),
    Color::RGBA(0, 128, 128, 0),
    Color::RGBA(128, 0, 128, 0),
    Color::RGBA(128, 128, 128, 0),
    Color::RGBA(64, 64, 64, 0),
    Color::RGBA(0, 255, 0, 0),
    Color::RGBA(255, 0, 0, 0),
    Color::RGBA(255, 255, 0, 0),
    Color::RGBA(0, 0, 255, 0),
    Color::RGBA(0, 255, 255, 0),
    Color::RGBA(255, 0, 255, 0),
    Color::RGBA(255, 255, 255, 0),
];

pub fn get_color(i: usize) -> Color {
    if i < 0 && i > PALETTE.len() - 1 {
        return PALETTE[0];
    }

    PALETTE[i]
}

// pub fn print_palette(canvas: &mut WindowCanvas) {
//     let mut x = 10;
//     let mut y = 10;
//
//     for i in PALETTE {
//         canvas.set_draw_color(i);
//         let rect = Rect::new(x, y, 25, 25);
//         canvas.draw_rect(rect).unwrap();
//         canvas.fill_rect(rect).unwrap();
//         x += 25;
//     }
//
//     x = 10;
//     y += 30;
//
//     for i in PALETTE_INTENSE {
//         canvas.set_draw_color(i);
//         let rect = Rect::new(x, y, 25, 25);
//         canvas.draw_rect(rect).unwrap();
//         canvas.fill_rect(rect).unwrap();
//         x += 25;
//     }
//
//     x = 10;
//     y += 30;
//
//     for i in PALETTE2 {
//         canvas.set_draw_color(i);
//         let rect = Rect::new(x, y, 25, 25);
//         canvas.draw_rect(rect).unwrap();
//         canvas.fill_rect(rect).unwrap();
//         x += 25;
//     }
// }
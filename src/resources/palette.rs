use sdl2::pixels::Color;

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

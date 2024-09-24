pub const HEIGHT: usize = 10;
pub const WIDTH: usize = 15;
pub const SIZE: usize = WIDTH * HEIGHT;

const S: u8 = 'S' as u8;
const B: u8 = 'B' as u8;
const H: u8 = 'H' as u8;
const V: u8 = 'V' as u8;
const C: u8 = 'C' as u8;
const G: u8 = ' ' as u8;

pub const LEVELS: [[[u8; WIDTH]; HEIGHT]; 8] = [
    [
        [S, G, G, G, B, G, G, G, G, G, H, H, H, H, S],
        [V, G, G, C, C, G, G, C, G, G, V, G, B, G, G],
        [V, B, G, C, C, G, G, C, G, G, V, G, G, G, G],
        [V, G, G, C, C, B, G, C, B, G, V, G, C, C, C],
        [V, G, G, C, C, G, G, C, G, G, V, G, C, C, C],
        [H, H, G, C, C, G, G, C, G, G, V, G, C, C, C],
        [G, V, G, G, G, G, B, G, B, G, V, G, G, G, G],
        [G, H, H, H, H, G, G, G, G, G, V, G, G, G, G],
        [C, G, G, G, V, G, G, G, G, G, V, G, G, G, C],
        [C, C, G, G, H, H, H, H, H, H, H, G, G, C, C],
    ],
    [
        [S, H, H, H, H, H, G, G, B, G, B, G, G, H, S],
        [G, C, C, G, G, V, G, G, G, G, G, G, G, V, G],
        [G, C, C, G, G, V, G, C, C, C, C, C, G, V, G],
        [B, C, C, B, G, V, G, C, C, C, C, C, G, V, G],
        [C, C, C, C, G, V, G, G, G, G, G, G, G, V, G],
        [C, C, C, C, G, V, G, B, G, G, H, H, H, H, G],
        [G, C, C, G, G, V, G, C, C, G, V, G, G, G, G],
        [G, B, B, G, G, V, C, C, C, C, V, G, C, C, G],
        [C, G, G, G, G, V, G, C, C, G, V, G, C, C, G],
        [C, C, G, G, G, H, H, H, H, H, H, G, G, G, G],
    ],
    [
        [S, H, H, H, H, B, G, B, G, B, H, H, H, H, S],
        [C, C, G, G, V, G, C, G, C, G, V, G, B, B, G],
        [C, G, G, G, V, G, C, G, C, G, V, G, C, C, G],
        [G, B, B, G, V, G, C, G, C, G, V, C, C, C, C],
        [C, C, C, C, V, G, C, G, C, G, V, C, C, C, C],
        [C, C, C, C, H, H, H, H, H, H, H, G, C, C, G],
        [G, C, C, G, G, C, G, V, G, C, G, G, C, C, G],
        [G, C, C, G, G, C, G, V, G, C, G, G, G, G, G],
        [C, G, G, G, G, C, G, V, G, C, G, G, G, G, C],
        [C, C, G, G, G, C, G, H, G, C, G, G, G, C, C],
    ],
    [
        [S, H, B, C, C, C, C, B, C, C, C, C, B, H, S],
        [C, V, G, G, C, C, C, C, C, C, C, G, G, V, C],
        [C, H, H, H, G, C, C, C, C, C, G, H, H, H, C],
        [C, G, G, V, G, G, C, C, C, G, G, V, G, G, C],
        [G, G, G, H, H, H, G, C, G, H, H, H, G, G, G],
        [G, G, B, G, G, V, G, B, G, V, G, G, B, G, G],
        [G, G, C, G, G, V, C, C, C, V, G, G, C, G, G],
        [G, C, C, C, G, H, H, H, H, H, G, C, C, C, G],
        [C, C, C, C, C, G, C, V, C, G, C, C, C, C, C],
        [C, C, C, C, C, G, C, H, C, G, C, C, C, C, C],
    ],
    [
        [S, H, H, H, H, H, H, H, H, H, H, H, H, H, S],
        [V, B, C, C, C, C, B, V, C, C, C, C, C, C, V],
        [V, C, C, C, C, C, C, V, G, C, C, B, C, G, V],
        [V, G, C, C, C, C, G, V, C, C, B, C, C, C, V],
        [V, C, C, C, C, C, C, V, G, C, C, C, C, G, V],
        [V, G, C, C, C, C, G, V, B, C, C, C, C, C, V],
        [V, C, C, B, C, C, C, V, G, C, C, C, C, G, V],
        [V, G, C, C, B, C, G, V, C, C, C, C, C, C, V],
        [V, C, C, C, C, C, C, V, C, C, C, C, C, C, V],
        [H, H, H, H, H, H, H, H, H, H, H, H, H, H, H],
    ],
    [
        [S, H, H, H, H, H, H, H, H, H, H, H, H, H, S],
        [V, C, B, C, C, V, G, V, G, V, C, C, B, C, V],
        [V, C, C, C, G, V, B, V, B, V, G, C, C, C, V],
        [V, C, C, C, H, H, G, V, G, H, H, C, C, C, V],
        [V, C, C, G, V, G, C, V, C, G, V, G, C, C, V],
        [V, C, C, H, H, G, C, V, C, G, H, H, C, C, V],
        [V, C, G, V, G, C, C, V, C, C, G, V, G, C, V],
        [V, C, H, H, B, C, C, V, C, C, B, H, H, C, V],
        [V, C, V, C, C, C, C, V, C, C, C, C, V, C, V],
        [H, H, H, H, H, H, H, H, H, H, H, H, H, H, H],
    ],
    [
        [S, H, C, C, C, C, C, V, C, C, C, C, C, H, S],
        [G, V, C, B, C, B, C, V, C, B, C, B, C, V, G],
        [B, V, C, C, C, C, C, V, C, C, C, C, C, V, B],
        [C, H, H, C, C, C, C, V, C, C, C, C, H, H, C],
        [C, C, V, G, C, C, C, V, C, C, C, G, V, C, C],
        [C, C, H, H, H, C, C, V, C, C, H, H, H, C, C],
        [C, C, C, C, V, G, C, V, C, G, V, C, C, C, C],
        [C, C, C, C, H, H, G, V, G, H, H, C, C, C, C],
        [C, C, C, C, C, V, G, V, G, V, C, C, C, C, C],
        [C, C, C, C, C, H, H, H, H, H, C, C, C, C, C],
    ],
    [
        [H, H, H, H, H, H, H, H, H, H, H, H, H, H, S],
        [V, G, C, C, B, C, C, C, C, C, B, C, C, G, V],
        [H, H, H, C, C, C, C, B, C, C, C, C, H, H, H],
        [V, B, V, G, C, C, C, C, C, C, C, G, V, B, V],
        [V, C, H, H, H, C, C, C, C, C, H, H, H, C, V],
        [V, C, C, B, V, G, C, C, C, G, V, B, C, C, V],
        [V, C, C, C, H, H, H, C, H, H, H, C, C, C, V],
        [V, C, C, C, C, G, V, G, V, G, C, C, C, C, V],
        [V, C, C, C, C, C, V, G, V, C, C, C, C, C, V],
        [H, H, H, H, H, H, H, H, H, H, H, H, H, H, H],
    ]
];

pub fn is_spawn(char: u8) -> bool {
    char == S
}

pub fn is_horizontal_path(char: u8) -> bool {
    char == H
}

pub fn is_vertical_path(char: u8) -> bool {
    char == V
}

pub fn is_bag(char: u8) -> bool {
    char == B
}

pub fn is_emerald(char: u8) -> bool {
    char == C
}

pub fn is_ground(char: u8) -> bool {
    char == G
}

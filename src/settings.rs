use crossterm::style::Color;

//
// Game constants
pub const BG_CHAR: char = '-';
pub const SHIP: char = '■';
pub const HIT: char = '#';
pub const STUNK: char = 'X';
pub const TABLE_SIZE: i32 = 10;
pub const TITLE: &str = "Battleship-rs";

//
// Utils
pub const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
// Colors
pub const GREY: Color = Color::Rgb {
    r: 150,
    g: 150,
    b: 150,
};

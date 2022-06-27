use crate::models::ship::Ship;
use crossterm::style::Color;
use std::collections::HashMap;
use std::time::Duration;

//
// Game constants
pub const BG_CHAR: char = '-';
pub const SHIP: char = '■';
pub const HIT: char = '#';
pub const STUNK: char = 'X';
pub const TABLE_SIZE: i32 = 10;
pub const TITLE: &str = "Battleship-rs";
pub const MOV_KEYS: [char; 4] = ['h', 'j', 'k', 'l'];

pub fn get_ships_left() -> HashMap<Ship, i32> {
    HashMap::from([
        (Ship::Aisle, 1),
        (Ship::Battleship, 1),
        (Ship::Cruiser, 1),
        (Ship::TorpedoBoat, 1),
        (Ship::Submarine, TABLE_SIZE - 9),
    ])
}

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

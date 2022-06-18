use crate::space::{Alignment, Vec2};

pub struct Ship {
    pub pos: Vec2<usize>,
    pub size: usize,
    pub aligment: Alignment,
    pub stunk: bool,
    pub hit: Vec<bool>,
}
impl Ship {
    pub fn new(size: usize) -> Ship {
        Ship {
            pos: Vec2 { x: 0, y: 0 },
            size,
            aligment: Alignment::Horizontal,
            stunk: false,
            hit: vec![false; size],
        }
    }
}

pub enum ShipType {
    Aisle,
    Battleship,
    Cruiser,
    TorpedoBoat,
    Submarine,
}

use crate::space::{Alignment, Vec2};

pub struct Ship {
    pub pos: Vec2<usize>,
    pub size: usize,
    pub aligment: Alignment,
    pub hit: Vec<bool>,
}
impl Ship {
    pub fn new(size: usize) -> Ship {
        Ship {
            pos: Vec2 { x: 0, y: 0 },
            size,
            aligment: Alignment::Horizontal,
            hit: vec![false; size],
        }
    }

    pub fn is_stunk(&self) -> bool {
        (*self.hit).into_iter().all(|v| *v)
    }
}

pub enum ShipType {
    Aisle,
    Battleship,
    Cruiser,
    TorpedoBoat,
    Submarine,
}

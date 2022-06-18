use crate::space::Vec2;

pub struct Ship {
    pub pos: Vec2<usize>,
    pub size: usize,
}
impl Ship {
    pub fn new(size: usize) -> Ship {
        Ship {
            pos: Vec2 { x: 0, y: 0 },
            size,
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

use crate::models::space::{Alignment, Vec2};

#[derive(Debug)]
pub struct Ship {
    pub pos: Vec2<i32>,
    pub size: i32,
    pub aligment: Alignment,
    pub hit: Vec<bool>,
}
impl Ship {
    pub fn new(ship_type: ShipType) -> Ship {
        let ship_size = ship_type.size();
        Ship {
            pos: Vec2 { x: 0, y: 0 },
            size: ship_size,
            aligment: Alignment::Horizontal,
            hit: vec![false; ship_size as usize],
        }
    }

    pub fn touch(&self, ship: &Ship) -> bool {
        match self.aligment {
            Alignment::Vertical => {
                (ship.pos.x >= self.pos.x - 1 && ship.pos.x <= self.pos.x + 1)
                    && (self.pos.y <= ship.pos.y + ship.size
                        && (ship.pos.y <= self.pos.y + self.size))
            }
            Alignment::Horizontal => {
                (ship.pos.y >= self.pos.y - 1 && ship.pos.y <= self.pos.y + 1)
                    && (self.pos.x <= ship.pos.x + ship.size
                        && (ship.pos.x <= self.pos.x + self.size))
            }
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
impl ShipType {
    pub fn size(&self) -> i32 {
        match &self {
            ShipType::Aisle => 5,
            ShipType::Battleship => 4,
            ShipType::Cruiser => 3,
            ShipType::TorpedoBoat => 2,
            ShipType::Submarine => 1,
        }
    }
}

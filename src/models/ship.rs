use std::hash::Hash;

#[derive(PartialEq, Eq, Hash)]
pub enum Ship {
    Aisle = 5,
    Battleship,
    Cruiser,
    TorpedoBoat,
    Submarine,
}
impl Ship {
    pub fn size(&self) -> i32 {
        match &self {
            Ship::Aisle => 5,
            Ship::Battleship => 4,
            Ship::Cruiser => 3,
            Ship::TorpedoBoat => 2,
            Ship::Submarine => 1,
        }
    }

    pub fn from_int(int: i32) -> Option<Ship> {
        match int {
            1 => Some(Ship::Submarine),
            2 => Some(Ship::TorpedoBoat),
            3 => Some(Ship::Cruiser),
            4 => Some(Ship::Battleship),
            5 => Some(Ship::Aisle),
            _ => None,
        }
    }
}

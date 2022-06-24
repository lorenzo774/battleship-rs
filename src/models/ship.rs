pub enum Ship {
    Aisle,
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
}

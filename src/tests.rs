#[cfg(test)]
pub mod tests {
    use crate::models::ship::Ship;
    use crate::models::space::{Alignment, Vec2};
    use crate::models::table::Table;
    use std::error::Error;

    #[test]
    pub fn insert_ships() -> Result<(), Box<dyn Error>> {
        let table = Table::new(Vec2::new(0, 0), 20);
        let mut ship = Ship::new(5);
        ship.pos = Vec2::new(14, 20);
        ship.aligment = Alignment::Horizontal;
        assert!(
            table.ship_out_of_boundaries(&ship) == true,
            "The ship should be out of boundaries because y > table size"
        );

        let mut ship = Ship::new(1);
        ship.pos = Vec2::new(19, 19);
        assert!(
            table.ship_out_of_boundaries(&ship) == false,
            "The ship should be not out of boundaries because the ship is in the bottom right corner of the table"
        );
        Ok(())
    }
}

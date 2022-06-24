#[cfg(test)]
pub mod tests {
    use crate::{
        lib::graphics::print_and_clear,
        models::{
            ship::{Ship, ShipType},
            space::{Alignment, Vec2},
            table::Table,
        },
    };
    use std::error::Error;

    #[test]
    pub fn insert_ships_in_different_pos() -> Result<(), Box<dyn Error>> {
        let table = Table::new(Vec2::new(0, 0), 20);
        let mut ship = Ship::new(ShipType::Aisle);
        ship.pos = Vec2::new(14, 20);
        ship.aligment = Alignment::Horizontal;
        assert!(
            table.ship_out_of_boundaries(&ship) == true,
            "The ship should be out of boundaries because y > table size"
        );

        let mut ship = Ship::new(ShipType::Submarine);
        ship.pos = Vec2::new(19, 19);
        assert!(
            table.ship_out_of_boundaries(&ship) == false,
            "The ship should be not out of boundaries because the ship is in the bottom right corner of the table"
        );
        Ok(())
    }

    #[test]
    pub fn insert_ship_next_to_other_vert() -> Result<(), Box<dyn Error>> {
        let mut table = Table::new(Vec2::new(0, 0), 20);
        table.insert_ship(ShipType::Battleship, Vec2::new(9, 0), &Alignment::Vertical)?;

        let mut ship = Ship::new(ShipType::Battleship);
        ship.pos = Vec2::new(9, 0);
        ship.aligment = Alignment::Vertical;

        let mut ship2 = Ship::new(ShipType::Battleship);
        ship2.pos = Vec2::new(7, 0);
        ship2.aligment = Alignment::Vertical;

        print_and_clear(format!("SHIPS = {:?}", table.ships))?;

        assert!(
            table.empty_for_ship(&ship2) == true,
            "The second ship should fit in the table, BOTTOM OF SECOND SHIP = {}",
            (ship.pos.y <= ship2.pos.y + ship2.size)
        );

        Ok(())
    }

    #[test]
    pub fn insert_ship_next_to_other_hor() -> Result<(), Box<dyn Error>> {
        let mut table = Table::new(Vec2::new(0, 0), 20);

        table.insert_ship(
            ShipType::Battleship,
            Vec2::new(9, 0),
            &Alignment::Horizontal,
        )?;

        let mut ship = Ship::new(ShipType::Battleship);
        ship.pos = Vec2::new(9, 0);
        ship.aligment = Alignment::Horizontal;

        let mut ship2 = Ship::new(ShipType::Battleship);
        ship2.pos = Vec2::new(7, 0);
        ship2.aligment = Alignment::Horizontal;

        print_and_clear(format!("SHIPS = {:?}", table.ships))?;

        assert!(
            table.empty_for_ship(&ship2) == true,
            "The second ship should fit in the table"
        );

        Ok(())
    }
}

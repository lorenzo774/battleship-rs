// #[cfg(test)]
// pub mod tests {
//     use crate::{
//         lib::graphics::print_and_clear,
//         models::{
//             ship::Ship,
//             space::{Alignment, Vec2},
//             table::Table,
//         },
//     };
//     use std::error::Error;

//     #[test]
//     pub fn insert_ships_in_different_pos() -> Result<(), Box<dyn Error>> {
//         let mut table = Table::new(Vec2::new(0, 0), 20);

//         table.insert_ship(Ship::Battleship, Vec2::new(9, 0), &Alignment::Vertical)?;

//         assert!(
//             table.insert_ship(Ship::Battleship, Vec2::new(9, 0), &Alignment::Vertical)?
//         == false,
//                    "The ship should be not out of boundaries because the ship is in the bottom right corner of the table"
//                );
//         Ok(())
//     }

//     #[test]
//     pub fn insert_ship_next_to_other_vert() -> Result<(), Box<dyn Error>> {
//         let mut table = Table::new(Vec2::new(0, 0), 20);
//         table.insert_ship(Ship::Battleship, Vec2::new(9, 0), &Alignment::Vertical)?;

//         assert!(
//             table.insert_ship(Ship::Battleship, Vec2::new(9, 0), &Alignment::Vertical)? == true,
//             "The second ship should fit in the table"
//         );

//         Ok(())
//     }

//     #[test]
//     pub fn insert_ship_next_to_other_hor() -> Result<(), Box<dyn Error>> {
//         let mut table = Table::new(Vec2::new(0, 0), 20);

//         table.insert_ship(Ship::Battleship, Vec2::new(9, 0), &Alignment::Horizontal)?;

//         assert!(
//             table.insert_ship(Ship::Battleship, Vec2::new(9, 0), &Alignment::Horizontal,)? == true,
//             "The second ship should fit in the table"
//         );

//         Ok(())
//     }
// }

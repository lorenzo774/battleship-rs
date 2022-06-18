use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    terminal::*,
};
use ship::ShipType;
use std::error::Error;
use std::io::stdout;

mod config;
mod ship;
mod space;
mod table;
mod utils;

use config::*;
use space::{Alignment, Vec2};
use table::Table;

// tests
mod tests;

fn main() -> Result<(), Box<dyn Error>> {
    let winning = false;

    // Create tables
    let mut player_table = Table::new(TABLE_SIZE);
    let mut com_table = Table::new(TABLE_SIZE);

    // Insert ships
    player_table.insert_ship(ShipType::Submarine, Vec2::new(9, 9), Alignment::Horizontal)?;
    com_table.insert_ship(ShipType::Battleship, Vec2::new(0, 3), Alignment::Vertical)?;

    // Clear screen
    execute!(stdout(), Clear(ClearType::All))?;

    // Game loop
    while !winning {
        execute!(stdout(), Hide, MoveTo(0, 0))?;

        // Render
        println!("Player");
        player_table.render()?;
        println!();
        println!("Computer");
        com_table.render()?;
    }

    Ok(())
}

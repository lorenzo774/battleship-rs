use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    terminal::*,
};
use models::ship::ShipType;
use std::error::Error;
use std::io::stdout;

mod config;
mod handlers;
mod lib;
mod models;
mod utils;

use config::*;
use handlers::input_handler::handle_inputs;
use models::space::{Alignment, Vec2};
use models::table::Table;
use utils::select_char;

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

    let mut select_pos: Vec2<usize> = Vec2::new(2, 15);

    // Game loop
    while !winning {
        execute!(stdout(), Hide, MoveTo(0, 0))?;

        // Render
        println!("Player");
        player_table.render(true)?;
        println!();
        println!("Computer");
        com_table.render(false)?;

        // Player pos
        select_char(crossterm::style::Color::Yellow, &select_pos)?;

        execute!(stdout(), MoveTo(0, 30))?;

        // Input handling
        println!();
        handle_inputs(&mut select_pos)?;
    }

    Ok(())
}

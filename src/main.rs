use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    terminal::*,
};
use ship::ShipType;
use std::io::stdout;
use std::{error::Error, process};

mod config;
mod inputs;
mod ship;
mod space;
mod table;
mod utils;

use config::*;
use inputs::get_input;
use space::{Alignment, Vec2};
use table::Table;
use utils::{print_and_clear, select_char};

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
        match get_input()? {
            // Exit
            'q' => {
                print_and_clear("Bye 👋\n".to_string())?;
                process::exit(0)
            }
            ' ' => print_and_clear("Nothing special is pressed".to_string())?,
            input => {
                print_and_clear("Move inputs".to_string())?;
                match input {
                    'j' => select_pos.y += 1,
                    'k' => select_pos.y -= 1,
                    'h' => select_pos.x -= 2,
                    'l' => select_pos.x += 2,
                    _ => (),
                };
                if select_pos.x > 2 * TABLE_SIZE {
                    select_pos.x = 2 * TABLE_SIZE;
                }
                if select_pos.x < 2 {
                    select_pos.x = 2;
                }
                if select_pos.y < 15 {
                    select_pos.y = 15;
                }
                if select_pos.y > 14 + TABLE_SIZE {
                    select_pos.y = 14 + TABLE_SIZE;
                }
            }
        };
    }

    Ok(())
}

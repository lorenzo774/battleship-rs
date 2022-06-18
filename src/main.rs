use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    terminal::*,
};
use std::io::{stdout, Error};

mod config;
mod ship;
mod table;
mod vector;

use config::*;
use table::Table;

fn main() -> Result<(), Error> {
    let winning = false;

    let mut player_table = Table::new(TABLE_SIZE);
    let mut com_table = Table::new(TABLE_SIZE);

    player_table.matrix[0][0] = SHIP;
    com_table.matrix[0][0] = SHIP;

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

use std::error::Error;

mod config;
mod handlers;
mod lib;
mod models;
mod utils;

use handlers::game_manager::*;
// tests
mod tests;

fn main() -> Result<(), Box<dyn Error>> {
    let winning = false;

    let mut game = Game::new();
    game.start()?;

    // Game loop
    while !winning {
        game.update()?;
    }

    Ok(())
}

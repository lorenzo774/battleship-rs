use std::error::Error;

use crate::handlers::game_manager::Game;

pub trait GameState {
    fn init(&mut self, game: &mut Game) -> Result<(), Box<dyn Error>>;
    /// Game loop
    fn run(&mut self, game: &mut Game) -> Result<(), Box<dyn Error>>;
    fn next(self: Box<Self>, game: &mut Game) -> Box<dyn GameState>;
}

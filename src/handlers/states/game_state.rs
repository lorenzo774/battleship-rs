use std::error::Error;

use crate::handlers::game_manager::Game;

pub trait GameState {
    fn init(&mut self, game: &mut Game) -> Result<(), Box<dyn Error>>;
    fn run(&mut self, game: &mut Game) -> Result<(), Box<dyn Error>>;
    fn next(self: Box<Self>) -> Box<dyn GameState>;
}

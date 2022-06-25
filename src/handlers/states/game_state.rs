use std::error::Error;

use crate::handlers::game_manager::Game;

pub trait GameState {
    fn init(&mut self, game: &mut Game);
    fn run(&mut self, game: &mut Game);
}

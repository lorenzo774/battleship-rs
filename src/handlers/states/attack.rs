use std::error::Error;

use crate::handlers::game_manager::Game;

use super::game_state::GameState;

pub struct AttackState {}

impl GameState for AttackState {
    fn init(
        &mut self,
        game: &mut crate::handlers::game_manager::Game,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn run(
        &mut self,
        game: &mut crate::handlers::game_manager::Game,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn next(self: Box<Self>, game: &mut Game) -> Box<dyn GameState> {
        todo!()
    }
}

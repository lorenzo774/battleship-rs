use std::{
    error::Error,
    io::{stdin, stdout},
};

use crate::{handlers::game_manager::Game, lib::inputs::get_input};

use super::game_state::GameState;

pub struct AttackState {}

impl GameState for AttackState {
    fn init(
        &mut self,
        game: &mut crate::handlers::game_manager::Game,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn run(
        &mut self,
        game: &mut crate::handlers::game_manager::Game,
    ) -> Result<(), Box<dyn Error>> {
        let mut ciao = String::new();
        stdin().read_line(&mut ciao);
        Ok(())
    }

    fn next(self: Box<Self>, game: &mut Game) -> Box<dyn GameState> {
        self
    }
}

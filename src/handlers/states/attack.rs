use std::{
    error::Error,
    io::{stdin, stdout},
    process,
};

use crossterm::event::KeyCode;

use crate::{
    handlers::game_manager::Game,
    lib::graphics::{clear_screen, print_and_clear},
    models::space::Vec2,
    settings::MOV_KEYS,
};

use super::game_state::GameState;

pub struct AttackState {}

impl GameState for AttackState {
    fn init(
        &mut self,
        game: &mut crate::handlers::game_manager::Game,
    ) -> Result<(), Box<dyn Error>> {
        game.select_pos = Vec2::new(0, 0);
        Ok(())
    }

    fn run(
        &mut self,
        game: &mut crate::handlers::game_manager::Game,
    ) -> Result<(), Box<dyn Error>> {
        // Render tables
        println!("Player");
        game.player_table.draw(true, None)?;
        println!("Computer");
        game.com_table.draw(false, Some(&game.select_pos))?;

        if let Some(key) = game.input_reader.get_key()? {
            match key {
                // TODO: Change for DRY
                KeyCode::Esc | KeyCode::Char('q') => {
                    clear_screen()?;
                    print_and_clear("Bye 👋\n".to_string())?;
                    process::exit(0)
                }
                KeyCode::Char(value) => {
                    if MOV_KEYS.contains(&value) {
                        game.move_char(value, true);
                    }
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn next(self: Box<Self>, game: &mut Game) -> Box<dyn GameState> {
        self
    }
}

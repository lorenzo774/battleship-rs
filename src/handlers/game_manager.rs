use std::io::stdout;

use crossterm::cursor::{Hide, MoveTo};
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use std::error::Error;

use crate::config::*;
use crate::handlers::input_manager::handle_inputs;
use crate::models::{space::Vec2, state::GameState, table::Table};
use crate::utils::{print_and_clear, select_char};

pub struct Game {
    player_table: Table,
    com_table: Table,
    select_pos: Vec2<usize>,
    state: GameState,
}
impl Game {
    pub fn new() -> Game {
        Game {
            player_table: Table::new(TABLE_SIZE),
            com_table: Table::new(TABLE_SIZE),
            select_pos: Vec2::new(0, 0),
            state: GameState::InsertShips,
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        // Set the first state
        self.change_state(GameState::InsertShips);

        // Clear screen
        execute!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    pub fn update(&mut self) -> Result<(), Box<dyn Error>> {
        execute!(stdout(), Hide, MoveTo(0, 0))?;
        // Render
        println!("Player");
        self.player_table.render(true)?;
        println!();
        println!("Computer");
        self.com_table.render(false)?;
        // Player pos
        select_char(crossterm::style::Color::Yellow, &self.select_pos)?;
        execute!(stdout(), MoveTo(0, 30))?;
        // Input handling
        println!();
        handle_inputs(&mut self.select_pos)?;
        self.handle_state()?;
        Ok(())
    }

    pub fn change_state(&mut self, new_state: GameState) {
        // Every change calls a function to handle the change of state
        match new_state {
            GameState::InsertShips => self.start_insert_ships(),
            GameState::Attack => todo!(),
            GameState::Win => todo!(),
            GameState::Loose => todo!(),
        }
        self.state = new_state;
    }

    pub fn handle_state(&self) -> Result<(), Box<dyn Error>> {
        match &self.state {
            GameState::InsertShips => print_and_clear("Insert ships state".to_string())?,
            GameState::Attack => todo!(),
            GameState::Win => todo!(),
            GameState::Loose => todo!(),
        };
        Ok(())
    }

    fn start_insert_ships(&mut self) {
        self.select_pos = Vec2::new(2, 2);
    }
}

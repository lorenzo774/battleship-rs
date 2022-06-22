use crossterm::cursor::{Hide, MoveTo};
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use std::error::Error;
use std::io::stdout;

use super::game_logic::handle_insertship_inputs;
use super::ui_manager::UI;

use crate::lib::graphics::{print_and_clear, select_char};
use crate::models::{space::Vec2, state::GameState, table::Table};
use crate::settings::*;

pub struct Game {
    player_table: Table,
    com_table: Table,
    select_pos: Vec2<usize>,
    state: GameState,
    ui: UI,
}
impl Game {
    pub fn new() -> Game {
        Game {
            player_table: Table::new(Vec2::new(2, 2), TABLE_SIZE),
            com_table: Table::new(Vec2::new(2, 15), TABLE_SIZE),
            select_pos: Vec2::new(0, 0),
            state: GameState::InsertShips,
            ui: UI::new(Vec2::new(40, 0), 10, 10),
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

        // Render tables
        println!("Player");
        self.player_table.draw(true)?;
        println!();
        println!("Computer");
        self.com_table.draw(false)?;
        select_char(crossterm::style::Color::Yellow, &self.select_pos)?;

        // UI
        self.ui.draw_title()?;

        // Move to debug ui
        execute!(stdout(), MoveTo(0, 30))?;

        // Input handling
        println!();
        self.handle_state()?;

        Ok(())
    }

    pub fn change_state(&mut self, new_state: GameState) {
        // Every change calls a function to handle the change of state
        match new_state {
            GameState::InsertShips => self.start_insert_ships(),
            GameState::Attack => self.start_attack_state(),
            GameState::Win => todo!(),
            GameState::Loose => todo!(),
        }
        self.state = new_state;
    }

    pub fn handle_state(&mut self) -> Result<(), Box<dyn Error>> {
        match &self.state {
            GameState::InsertShips => {
                print_and_clear("Insert ships state".to_string())?;
                handle_insertship_inputs(&mut self.player_table, &mut self.select_pos)?;
            }
            GameState::Attack => {
                print_and_clear("Attack state".to_string())?;
                handle_insertship_inputs(&mut self.com_table, &mut self.select_pos)?;
            }
            GameState::Win => todo!(),
            GameState::Loose => todo!(),
        };
        Ok(())
    }

    fn start_insert_ships(&mut self) {
        self.select_pos = Vec2::new(2, 2);
    }

    fn start_attack_state(&mut self) {
        self.select_pos = Vec2::new(2, 15);
    }
}

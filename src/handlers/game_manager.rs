use crossterm::cursor::{Hide, MoveTo};
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use std::error::Error;
use std::io::stdout;

use super::states::game_state::GameState;
use super::states::insert_ships::InsertShips;
use super::ui_manager::UI;

use crate::lib::graphics::select_char;
use crate::models::{space::Vec2, table::Table};
use crate::settings::*;

pub struct Game {
    com_table: Table,
    state: Option<Box<dyn GameState>>,

    pub player_table: Table,
    pub select_pos: Vec2<i32>,
    pub ui: UI,
}
impl Game {
    pub fn new() -> Game {
        Game {
            player_table: Table::new(Vec2::new(2, 2), TABLE_SIZE),
            com_table: Table::new(Vec2::new(2, 15), TABLE_SIZE),
            select_pos: Vec2::new(0, 0),
            state: Some(Box::new(InsertShips::new())),
            ui: UI::new(Vec2::new(40, 6), 20, 14),
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        // TODO: Reset game
        if let Some(mut s) = self.state.take() {
            s.init(self)?;
            self.state = Some(s);
        }
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

        println!();
        // Handle state
        if let Some(mut s) = self.state.take() {
            s.run(self)?;
            self.state = Some(s.next(self));
        }
        // Draw title
        self.ui.draw_msg(0, TITLE.to_string())?;
        Ok(())
    }

    pub fn move_char(&mut self, input: char, player_table: bool) {
        match input {
            'j' => self.select_pos.y += 1,
            'k' => self.select_pos.y -= 1,
            'h' => self.select_pos.x -= 2,
            'l' => self.select_pos.x += 2,
            _ => (),
        };
        let table = match player_table {
            true => &self.player_table,
            false => &self.com_table,
        };
        table.rect.set_in_boundaries(&mut self.select_pos);
    }
}

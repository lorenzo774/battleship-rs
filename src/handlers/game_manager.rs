use crossterm::cursor::{Hide, MoveTo};
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use std::error::Error;
use std::io::stdout;
use std::process;

use super::ui_manager::UI;

use crate::lib::graphics::{clear_screen, print_and_clear, select_char};
use crate::lib::inputs::get_input;
use crate::models::rect::Rect;
use crate::models::ship::Ship;
use crate::models::space::Alignment;
use crate::models::{space::Vec2, state::GameState, table::Table};
use crate::settings::*;

pub struct Game {
    player_table: Table,
    com_table: Table,
    select_pos: Vec2<i32>,
    state: GameState,
    ui: UI,
    cur_orientation: Alignment,
}
impl Game {
    pub fn new() -> Game {
        Game {
            player_table: Table::new(Vec2::new(2, 2), TABLE_SIZE),
            com_table: Table::new(Vec2::new(2, 15), TABLE_SIZE),
            select_pos: Vec2::new(0, 0),
            state: GameState::InsertShips,
            ui: UI::new(Vec2::new(40, 0), 10, 10),
            cur_orientation: Alignment::Horizontal,
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
        match &mut self.state {
            GameState::InsertShips => {
                self.ui.draw_ship(&Ship::Aisle)?;
                match get_input()? {
                    // Exit
                    'q' => {
                        clear_screen()?;
                        print_and_clear("Bye 👋\n".to_string())?;
                        process::exit(0)
                    }
                    'E' => self.handle_insert_new_ship()?,
                    'H' => self.cur_orientation = Alignment::Horizontal,
                    'V' => self.cur_orientation = Alignment::Vertical,
                    ' ' => (),
                    input => {
                        self.move_char(input, true);
                    }
                };
                self.ui.draw_orientation(&&self.cur_orientation)?;
            }
            GameState::Attack => {
                // handle_insertship_inputs(&mut self.com_table, &mut self.select_pos)?;
            }
            GameState::Win => todo!(),
            GameState::Loose => todo!(),
        };
        Ok(())
    }

    fn move_char(&mut self, input: char, player_table: bool) {
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

    // Handle insert new ship
    fn handle_insert_new_ship(&mut self) -> Result<(), Box<dyn Error>> {
        let table_pos = match Rect::convert_to_rect_pos(&self.select_pos, &self.player_table.rect) {
            Some(rect_pos) => rect_pos,
            None => Vec2::new(0, 0),
        };

        let res =
            &self
                .player_table
                .insert_ship(Ship::TorpedoBoat, table_pos, &self.cur_orientation);

        match res {
            Ok(_) => {}
            Err(msg) => (),
        }
        Ok(())
    }

    fn start_insert_ships(&mut self) {
        self.select_pos = Vec2::new(2, 2);
    }

    fn start_attack_state(&mut self) {
        self.select_pos = Vec2::new(2, 15);
    }
}

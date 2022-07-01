use crossterm::cursor::{Hide, MoveTo};
use crossterm::execute;
use std::error::Error;
use std::io::stdout;

use super::states::game_state::GameState;
use super::states::insert_ships::InsertShips;

use crate::lib::canvas::Canvas;
use crate::lib::graphics::clear_screen;
use crate::lib::inputs::InputReader;
use crate::models::{space::Vec2, table::Table};
use crate::settings::*;
use crate::widgets::ui_container::UIContainer;

pub struct Game {
    state: Option<Box<dyn GameState>>,

    pub canvas: Canvas,
    pub com_table: Table,
    pub player_table: Table,
    pub select_pos: Vec2<i32>,
    pub ui: UIContainer,
    pub input_reader: InputReader,
}
impl Game {
    pub fn new(input_reader: InputReader) -> Game {
        Game {
            player_table: Table::new(Vec2::new(2, 2), TABLE_SIZE),
            com_table: Table::new(Vec2::new(2, 15), TABLE_SIZE),
            select_pos: Vec2::new(0, 0),
            state: Some(Box::new(InsertShips::new())),
            ui: UIContainer::new(Vec2::new(40, 0), 20, 15),
            input_reader,
            canvas: Canvas::new(),
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        // Clear screen
        clear_screen()?;

        // Insert widgets for game manager
        // self.ui.insert_widget(Label::new());
        // TODO: Reset game
        if let Some(mut s) = self.state.take() {
            s.init(self)?;
            self.state = Some(s);
        }
        Ok(())
    }

    pub fn update(&mut self) -> Result<(), Box<dyn Error>> {
        execute!(stdout(), Hide, MoveTo(0, 0))?;
        // TODO: Handle the exit system without clone the code for every state
        // handle_exit(&self.input_reader)?;
        if let Some(mut s) = self.state.take() {
            s.run(self)?;
            self.state = Some(s.next(self));
        }
        // Insert surfs

        self.canvas.surfs = vec![
            // self.ui.get_surface(),
            self.player_table.get_surface(true).unwrap(),
        ];

        self.canvas.update_canvas();
        self.canvas.draw_canvas()?;
        // self.ui.draw_msg(0, TITLE.to_string())?;
        Ok(())
    }

    pub fn move_char(&mut self, input: char, player_table: bool) {
        match input {
            'j' => self.select_pos.y += 1,
            'k' => self.select_pos.y -= 1,
            'h' => self.select_pos.x -= 1,
            'l' => self.select_pos.x += 1,
            _ => (),
        };
        let table = match player_table {
            true => &self.player_table,
            false => &self.com_table,
        };
        table.rect.set_in_boundaries(&mut self.select_pos);
    }
}

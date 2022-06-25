use std::{error::Error, process};

use super::game_state::GameState;
use crate::{
    handlers::game_manager::Game,
    lib::{
        graphics::{clear_screen, print_and_clear},
        inputs::get_input,
    },
    models::{
        rect::Rect,
        ship::Ship,
        space::{Alignment, Vec2},
    },
};

pub struct InsertShips {
    cur_orientation: Alignment,
}
// State definition
impl GameState for InsertShips {
    fn init(&mut self, game: &mut Game) {
        game.select_pos = Vec2::new(2, 2);
    }

    fn run(&mut self, game: &mut Game) {
        game.ui.draw_ship(&Ship::Aisle);
        match get_input().unwrap() {
            // Exit
            'q' => {
                clear_screen();
                print_and_clear("Bye 👋\n".to_string());
                process::exit(0)
            }
            'E' => self.handle_insert_new_ship(game),
            'H' => self.cur_orientation = Alignment::Horizontal,
            'V' => self.cur_orientation = Alignment::Vertical,
            ' ' => (),
            input => game.move_char(input, true),
        };
        game.ui.draw_orientation(&&self.cur_orientation);
    }
}
// Struct methods
impl InsertShips {
    pub fn new() -> InsertShips {
        InsertShips {
            cur_orientation: Alignment::Horizontal,
        }
    }

    fn handle_insert_new_ship(&self, game: &mut Game) {
        let table_pos = match Rect::convert_to_rect_pos(&game.select_pos, &game.player_table.rect) {
            Some(rect_pos) => rect_pos,
            None => Vec2::new(0, 0),
        };

        let res =
            game.player_table
                .insert_ship(Ship::TorpedoBoat, table_pos, &self.cur_orientation);

        match res {
            Ok(_) => {}
            Err(msg) => (),
        }
    }
}

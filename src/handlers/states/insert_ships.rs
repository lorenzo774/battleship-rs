use std::{collections::HashMap, error::Error, process};

use super::attack::AttackState;
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
    settings::TABLE_SIZE,
};

pub struct InsertShips {
    ship_counter: i32,
    selected_ship: Ship,
    cur_orientation: Alignment,
    ships_left: HashMap<Ship, i32>,
}
// State definition
impl GameState for InsertShips {
    fn init(&mut self, game: &mut Game) -> Result<(), Box<dyn Error>> {
        game.select_pos = Vec2::new(2, 2);
        Ok(())
    }

    fn run(&mut self, game: &mut Game) -> Result<(), Box<dyn Error>> {
        match get_input()? {
            // Exit
            'q' => {
                clear_screen()?;
                print_and_clear("Bye 👋\n".to_string())?;
                process::exit(0)
            }
            'E' => self.handle_insert_new_ship(game),
            'H' => self.cur_orientation = Alignment::Horizontal,
            'V' => self.cur_orientation = Alignment::Vertical,
            'A' => self.change_selected_ship(-1),
            'D' => self.change_selected_ship(1),
            ' ' => (),
            input => game.move_char(input, true),
        };
        game.ui
            .draw_selected_ship(&&self.selected_ship, &&self.cur_orientation)?;
        game.ui
            .draw_msg(9, self.ships_left[&self.selected_ship].to_string())?;
        game.ui.draw_orientation(&&self.cur_orientation)?;
        Ok(())
    }

    fn next(self: Box<Self>, game: &mut Game) -> Box<dyn GameState> {
        if self.no_ships_left() {
            InsertShips::generate_com_ships(game);
            return Box::new(AttackState {});
        }
        self
    }
}
// Struct methods
impl InsertShips {
    pub fn new() -> InsertShips {
        InsertShips {
            ship_counter: 1,
            selected_ship: Ship::Aisle,
            cur_orientation: Alignment::Horizontal,
            ships_left: HashMap::from([
                (Ship::Aisle, 1),
                (Ship::Battleship, 2),
                (Ship::Cruiser, 2),
                (Ship::TorpedoBoat, 4),
                (Ship::Submarine, TABLE_SIZE - 9),
            ]),
        }
    }

    fn change_selected_ship(&mut self, change: i32) {
        self.ship_counter += change;

        if self.ship_counter > 5 {
            self.ship_counter = 1;
        }
        if self.ship_counter < 1 {
            self.ship_counter = 5;
        }

        if let Some(v) = Ship::from_int(self.ship_counter) {
            self.selected_ship = v;
        }
    }

    fn no_ships_left(&self) -> bool {
        self.ships_left.values().all(|&x| x <= 0)
    }

    fn handle_insert_new_ship(&mut self, game: &mut Game) {
        if let Some(v) = self.ships_left.get_mut(&self.selected_ship) {
            if *v == 0 {
                return;
            }
        };

        let table_pos = match Rect::convert_to_rect_pos(&game.select_pos, &game.player_table.rect) {
            Some(rect_pos) => rect_pos,
            None => Vec2::new(0, 0),
        };

        let res =
            game.player_table
                .insert_ship(&self.selected_ship, table_pos, &self.cur_orientation);

        if let Ok(_) = res {
            if let Some(v) = self.ships_left.get_mut(&self.selected_ship) {
                *v -= 1;
            };
        }
    }
    // Generate ships in random position in the computer table
    fn generate_com_ships(game: &mut Game) {}
}

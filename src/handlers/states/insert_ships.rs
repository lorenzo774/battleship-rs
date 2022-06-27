use std::{collections::HashMap, error::Error, process};

use super::attack::AttackState;
use super::game_state::GameState;
use crate::{
    handlers::game_manager::Game,
    lib::graphics::{clear_screen, print_and_clear},
    models::{
        rect::Rect,
        ship::Ship,
        space::{Alignment, Vec2},
        table::Table,
    },
    settings::{get_ships_left, MOV_KEYS, TABLE_SIZE},
};

use crossterm::event::KeyCode;

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
        if let Some(key) = game.input_reader.get_key()? {
            match key {
                // TODO: Change for DRY
                KeyCode::Esc | KeyCode::Char('q') => {
                    clear_screen()?;
                    print_and_clear("Bye 👋\n".to_string())?;
                    process::exit(0)
                }
                KeyCode::Enter => self.handle_insert_new_ship(game),
                KeyCode::Char(value) => match value {
                    'w' => self.cur_orientation = Alignment::Vertical,
                    's' => self.cur_orientation = Alignment::Horizontal,
                    'a' => self.change_selected_ship(-1),
                    'd' => self.change_selected_ship(1),
                    other_key => {
                        if MOV_KEYS.contains(&other_key) {
                            game.move_char(other_key, true);
                        }
                    }
                },
                _ => (),
            }
        }
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
            ships_left: get_ships_left(),
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
    fn generate_com_ships(game: &mut Game) {
        let ships_left = get_ships_left();

        for ship in ships_left.keys() {
            for _ in 0..ships_left[ship] {
                InsertShips::gen_random_ship(ship, &mut game.com_table);
            }
        }
    }

    fn gen_random_ship(ship: &Ship, com_table: &mut Table) {
        loop {
            let rand_pos = Vec2::new(
                (rand::random::<u32>() % (TABLE_SIZE - ship.size()) as u32) as i32,
                (rand::random::<u32>() % (TABLE_SIZE - ship.size()) as u32) as i32,
            );
            let rand_aligment = match rand::random() {
                true => Alignment::Horizontal,
                false => Alignment::Vertical,
            };
            if let Err(_) = com_table.insert_ship(&ship, rand_pos, &rand_aligment) {
                continue;
            }
            break;
        }
    }
}

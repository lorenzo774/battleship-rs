use crate::{
    config::{ALPHABET, BG_CHAR, GREY, HIT, SHIP, STUNK},
    ship::{Ship, ShipType},
    space::{Alignment, Vec2},
    utils::print_color,
};
use crossterm::{
    execute,
    style::{Color, Print},
};
use std::io::{stdout, Error};

pub struct Table {
    pub matrix: Vec<Vec<char>>,
    pub ships: Vec<Ship>,
    pub size: usize,
}
impl Table {
    /// Generate a new matrix for the table and a ships vector
    pub fn new(size: usize) -> Table {
        let mut matrix: Vec<Vec<char>> = Vec::new();

        for i in 0..size {
            matrix.push(Vec::new());
            for _ in 0..size {
                matrix[i].push(BG_CHAR);
            }
        }

        Table {
            size,
            matrix,
            ships: Vec::new(),
        }
    }

    /// Check if the ship is out of boundaries
    pub fn ship_out_of_boundaries(&self, ship: &Ship) -> bool {
        match ship.aligment {
            Alignment::Vertical => {
                ship.pos.y + ship.size - 1 >= self.size || ship.pos.x >= self.size
            }
            Alignment::Horizontal => {
                ship.pos.x + ship.size - 1 >= self.size || ship.pos.y >= self.size
            }
        }
    }

    /// Insert a new ship into the table by a ship type
    pub fn insert_ship(
        &mut self,
        ship: ShipType,
        pos: Vec2<usize>,
        aligment: Alignment,
    ) -> Result<(), String> {
        let mut new_ship = match ship {
            ShipType::Aisle => Ship::new(5),
            ShipType::Battleship => Ship::new(4),
            ShipType::Cruiser => Ship::new(3),
            ShipType::TorpedoBoat => Ship::new(2),
            ShipType::Submarine => Ship::new(1),
        };
        new_ship.aligment = aligment;
        new_ship.pos = pos;
        if self.ship_out_of_boundaries(&new_ship) {
            return Err(format!(
                "The ship is out of boundaries ({:?})",
                new_ship.pos
            ));
        }
        // Continue if the ship is in the table
        self.ships.push(new_ship);
        Ok(())
    }

    // Clean the matrix
    pub fn reset_matrix(&mut self) {
        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[i].len() {
                self.matrix[i][j] = BG_CHAR;
            }
        }
    }

    // Here the ships is supposed to be in boundaries
    fn insert_ships_to_matrix(&mut self, ships: bool) {
        for ship in &self.ships {
            for i in 0..ship.size {
                if ship.is_stunk() {
                    self.matrix[ship.pos.x][ship.pos.y + i] = STUNK;
                    continue;
                }
                match ship.aligment {
                    Alignment::Vertical => {
                        self.matrix[ship.pos.x][ship.pos.y + i] = match ship.hit[i] {
                            true => HIT,
                            false => {
                                if ships {
                                    SHIP
                                } else {
                                    BG_CHAR
                                }
                            }
                        };
                    }
                    Alignment::Horizontal => {
                        self.matrix[ship.pos.x + i][ship.pos.y] = match ship.hit[i] {
                            true => HIT,
                            false => {
                                if ships {
                                    SHIP
                                } else {
                                    BG_CHAR
                                }
                            }
                        };
                    }
                };
            }
        }
    }

    /// Print the table to the console
    pub fn render(&mut self, ships: bool) -> Result<(), Error> {
        self.reset_matrix();

        self.insert_ships_to_matrix(ships);
        execute!(stdout(), Print("x "))?;
        for k in 1..self.matrix.len() + 1 {
            print_color(format!("{} ", k), Color::Blue)?;
        }
        println!();
        for i in 0..self.matrix.len() {
            print_color(format!("{} ", ALPHABET[i]), Color::Green)?;
            for j in 0..self.matrix[i].len() {
                let color = match self.matrix[i][j] {
                    SHIP => Color::DarkMagenta,
                    HIT => GREY,
                    _ => Color::Cyan,
                };
                print_color(format!("{} ", self.matrix[i][j]), color)?;
            }
            println!();
        }
        Ok(())
    }
}

use crate::{
    lib::graphics::print_color,
    models::ship::Ship,
    models::space::{Alignment, Vec2},
    settings::{ALPHABET, BG_CHAR, GREY, HIT, SHIP, STUNK},
};
use crossterm::{
    execute,
    style::{Color, Print},
};
use std::io::{stdout, Error};

use super::rect::Rect;

pub struct Table {
    pub matrix: Vec<Vec<char>>,
    pub rect: Rect,
    pub size: i32,
}
impl Table {
    /// Generate a new matrix for the table and a ships vector
    pub fn new(pos: Vec2<i32>, size: i32) -> Table {
        let mut matrix: Vec<Vec<char>> = Vec::new();
        let rect = Rect::new(pos, size * 2, size);

        for i in 0..size {
            matrix.push(Vec::new());
            for _ in 0..size {
                matrix[i as usize].push(BG_CHAR);
            }
        }

        Table { size, matrix, rect }
    }

    pub fn ship_out_of_boundaries(
        &self,
        ship: &Ship,
        pos: &Vec2<i32>,
        aligment: &Alignment,
    ) -> bool {
        match aligment {
            Alignment::Vertical => pos.y + ship.size() - 1 >= self.size || pos.x >= self.size,
            Alignment::Horizontal => pos.x + ship.size() - 1 >= self.size || pos.y >= self.size,
        }
    }

    fn empty_for_new_ship(&self, ship: &Ship, pos: &Vec2<i32>, aligment: &Alignment) -> bool {
        match aligment {
            Alignment::Vertical => {
                for i in -1..ship.size() + 1 {
                    for j in -1..2 {
                        // print_and_clear(format!("i = {}, j = {} yes = {}", i, j, pos.y + i < 0));
                        if pos.y + i < 0 || pos.y + i >= self.size {
                            continue;
                        }
                        if pos.x + j < 0 || pos.x + j >= self.size {
                            continue;
                        }
                        if self.matrix[(pos.y + i) as usize][(pos.x + j) as usize] == SHIP {
                            return false;
                        }
                    }
                }
            }
            Alignment::Horizontal => {
                for i in -1..ship.size() + 1 {
                    for j in -1..2 {
                        if pos.y + j < 0 || pos.y + j >= self.size {
                            continue;
                        }
                        if pos.x + i < 0 || pos.x + i >= self.size {
                            continue;
                        }
                        if self.matrix[((pos.y + j) as usize)][(pos.x + i) as usize] == SHIP {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    /// Insert a new ship into the table by a ship type
    pub fn insert_ship(
        &mut self,
        ship: Ship,
        pos: Vec2<i32>,
        aligment: &Alignment,
    ) -> Result<bool, String> {
        if self.ship_out_of_boundaries(&ship, &pos, &aligment) {
            return Ok(false);
        }
        if !self.empty_for_new_ship(&ship, &pos, &aligment) {
            return Ok(false);
        }

        match aligment {
            Alignment::Vertical => {
                for i in 0..ship.size() {
                    self.matrix[pos.y as usize + i as usize][pos.x as usize] = SHIP;
                }
            }
            Alignment::Horizontal => {
                for i in 0..ship.size() {
                    self.matrix[pos.y as usize][pos.x as usize + i as usize] = SHIP;
                }
            }
        }
        Ok(true)
    }

    /// Print the table to the console
    pub fn draw(&mut self, ships: bool) -> Result<(), Error> {
        execute!(stdout(), Print("x "))?;
        for k in 1..self.matrix.len() + 1 {
            print_color(format!("{} ", k), Color::Blue)?;
        }
        println!();
        for i in 0..self.matrix.len() {
            print_color(format!("{} ", ALPHABET[i]), Color::Green)?;
            for j in 0..self.matrix[i].len() {
                if !ships && self.matrix[i][j] == SHIP {
                    print_color("- ".to_string(), Color::Cyan)?;
                    continue;
                }
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

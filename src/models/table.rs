use crate::{
    lib::surface::Surface,
    models::ship::Ship,
    models::space::{Alignment, Vec2},
    settings::{ALPHABET, BG_CHAR, GREY, HIT, SHIP, STUNK},
};
use crossterm::style::Color;
use std::io::Error;

use super::rect::Rect;

pub struct Table {
    pub matrix: Vec<Vec<i8>>,
    pub rect: Rect,
    pub size: i32,
}
impl Table {
    /// Generate a new matrix for the table and a ships vector
    pub fn new(pos: Vec2<i32>, size: i32) -> Table {
        let mut matrix: Vec<Vec<i8>> = Vec::new();
        let rect = Rect::new(pos, size, size);

        for i in 0..size {
            matrix.push(Vec::new());
            for _ in 0..size {
                matrix[i as usize].push(0);
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
                        if pos.y + i < 0 || pos.y + i >= self.size {
                            continue;
                        }
                        if pos.x + j < 0 || pos.x + j >= self.size {
                            continue;
                        }
                        if self.matrix[(pos.y + i) as usize][(pos.x + j) as usize] != 0 {
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
                        if self.matrix[((pos.y + j) as usize)][(pos.x + i) as usize] != 0 {
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
        ship: &Ship,
        pos: &Vec2<i32>,
        aligment: &Alignment,
    ) -> Result<(), String> {
        if self.ship_out_of_boundaries(&ship, &pos, &aligment) {
            return Err("Ship out of boundaries".to_string());
        }
        if !self.empty_for_new_ship(&ship, &pos, &aligment) {
            return Err("You can't insert a ship here".to_string());
        }

        match aligment {
            Alignment::Vertical => {
                for i in 0..ship.size() {
                    self.matrix[pos.y as usize + i as usize][pos.x as usize] = ship.size() as i8;
                }
            }
            Alignment::Horizontal => {
                for i in 0..ship.size() {
                    self.matrix[pos.y as usize][pos.x as usize + i as usize] = ship.size() as i8;
                }
            }
        }
        Ok(())
    }

    /// Print the table to the console
    pub fn get_surface(&mut self, ships: bool) -> Result<Surface, Error> {
        let mut surface = Surface::new(
            self.rect.pos.clone(),
            ((self.size * 2) + 1, (self.size * 2) + 3),
        );
        surface
            .texture
            .change_m_pixel(0, 0, Some(String::from("X ")), Some(Color::Black));
        for k in 1..self.matrix.len() + 1 {
            surface.texture.change_m_pixel(
                k as i32 * 2,
                0,
                Some(format!("{} ", k)),
                Some(Color::Blue),
            );
        }
        for i in 0..self.matrix.len() {
            surface
                .texture
                .change_pixel(0, i as i32 + 1, Some(ALPHABET[i]), Some(Color::Green));
            for j in 0..self.matrix[i].len() {
                if !ships {
                    surface.texture.change_pixel(
                        (j as i32 + 1) * 2,
                        i as i32 + 1,
                        Some(BG_CHAR),
                        Some(Color::Cyan),
                    );
                    continue;
                }
                let color = match self.matrix[i][j] {
                    -1 => GREY,
                    0 => Color::Cyan,
                    _ => Color::DarkMagenta,
                };
                let char = match self.matrix[i][j] {
                    0 => BG_CHAR,
                    -1 => HIT,
                    _ => SHIP,
                };
                surface.texture.change_pixel(
                    (j as i32 + 1) * 2,
                    i as i32 + 1,
                    Some(char),
                    Some(color),
                );
            }
        }
        Ok(surface)
    }
}

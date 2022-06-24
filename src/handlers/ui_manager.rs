use crate::lib::graphics::{print_clear_color_at, print_color_at};
use crate::models::rect::Rect;
use crate::models::ship::Ship;
use crate::models::space::{Alignment, Vec2};
use crate::settings::{SHIP, TITLE};

use crossterm::style::Color;
use std::error::Error;

/// UI struct
pub struct UI {
    rect: Rect,
}
impl UI {
    pub fn new(pos: Vec2<i32>, width: i32, height: i32) -> UI {
        UI {
            rect: Rect::new(pos, width, height),
        }
    }

    pub fn draw_title(&self) -> Result<(), Box<dyn Error>> {
        print_color_at(&self.rect.pos, TITLE.to_string(), Color::White)?;
        Ok(())
    }

    pub fn draw_ship(&self, ship: &Ship) -> Result<(), Box<dyn Error>> {
        let start_pos = Vec2::new(self.rect.pos.x + (self.rect.width / 2), self.rect.pos.y + 1);

        for i in 0..ship.size() {
            let ship_pos = Vec2::new(start_pos.x, start_pos.y + i + 1);
            print_color_at(&ship_pos, SHIP.to_string(), Color::Blue)?;
        }
        Ok(())
    }

    pub fn draw_orientation(&self, orientation: &Alignment) -> Result<(), Box<dyn Error>> {
        let msg = format!(
            "Orientation: {}",
            match orientation {
                Alignment::Vertical => "vertical",
                Alignment::Horizontal => "horizontal",
            }
        );
        let draw_pos = Vec2::new(
            self.rect.pos.x + self.rect.width - (msg.len() as i32 / 2),
            self.rect.pos.y + self.rect.height,
        );
        print_clear_color_at(&draw_pos, msg, Color::Red)?;

        Ok(())
    }
}

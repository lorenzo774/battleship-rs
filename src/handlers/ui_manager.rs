use crate::lib::graphics::print_color_at;
use crate::models::rect::Rect;
use crate::models::space::Vec2;
use crate::settings::TITLE;

use crossterm::style::Color;
use std::error::Error;

/// UI struct
pub struct UI {
    rect: Rect,
}
impl UI {
    pub fn new(pos: Vec2<usize>, width: usize, height: usize) -> UI {
        UI {
            rect: Rect::new(pos, width, height),
        }
    }

    pub fn draw_title(&self) -> Result<(), Box<dyn Error>> {
        print_color_at(
            &self.rect.pos,
            TITLE.to_string(),
            Color::White,
            Color::Black,
        )?;
        Ok(())
    }
}

use crate::lib::graphics::print_color_at;
use crate::models::rect::Rect;
use crate::models::ship::Ship;
use crate::models::space::{Alignment, Vec2};
use crate::settings::SHIP;

use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::Color;
use crossterm::terminal::{Clear, ClearType};
use std::error::Error;
use std::io::stdout;

/// Handle UI for msg
pub struct UI {
    rect: Rect,
}
impl UI {
    pub fn new(pos: Vec2<i32>, width: i32, height: i32) -> UI {
        UI {
            rect: Rect::new(pos, width, height),
        }
    }

    pub fn clear(&self) {
        for i in 0..self.rect.height {
            execute!(
                stdout(),
                MoveTo(self.rect.pos.x as u16, (self.rect.pos.y + i) as u16),
                Clear(ClearType::UntilNewLine)
            )
            .unwrap();
        }
    }

    pub fn draw_msg(&self, y: i32, msg: String) -> Result<(), Box<dyn Error>> {
        let draw_pos = Vec2::new(
            self.rect.pos.x + (self.rect.width / 2) - (msg.len() as i32 / 2),
            self.rect.pos.y + y,
        );
        print_color_at(&draw_pos, msg.to_string(), Color::White)?;
        Ok(())
    }

    pub fn draw_selected_ship(
        &self,
        ship: &Ship,
        alignment: &Alignment,
    ) -> Result<(), Box<dyn Error>> {
        let get_start_pos: Box<fn(&Rect, &Alignment) -> Vec2<i32>> =
            Box::new(|rect: &Rect, alignment: &Alignment| -> Vec2<i32> {
                match alignment {
                    Alignment::Vertical => Vec2::new(
                        rect.pos.x + (rect.width / 2),
                        rect.pos.y + (rect.height / 5) - 1,
                    ),
                    Alignment::Horizontal => {
                        Vec2::new(rect.pos.x + 5, rect.pos.y + (rect.height / 5) + 2)
                    }
                }
            });

        let calc_ship_pos: Box<fn(i32, &Vec2<i32>, &Alignment) -> Vec2<i32>> = Box::new(
            |i: i32, start_pos: &Vec2<i32>, aligment: &Alignment| -> Vec2<i32> {
                match aligment {
                    Alignment::Vertical => Vec2::new(start_pos.x, start_pos.y + i + 1),
                    Alignment::Horizontal => Vec2::new(start_pos.x + (i * 2) + 1, start_pos.y),
                }
            },
        );
        // Clear
        self.draw_line(
            &get_start_pos,
            &Alignment::Horizontal,
            ' ',
            5,
            &calc_ship_pos,
        )?;
        self.draw_line(
            &get_start_pos,
            &&Alignment::Vertical,
            ' ',
            5,
            &calc_ship_pos,
        )?;
        // Draw ship
        self.draw_line(&get_start_pos, alignment, SHIP, ship.size(), &calc_ship_pos)?;
        Ok(())
    }

    fn draw_line(
        &self,
        start_pos: &Box<fn(&Rect, &Alignment) -> Vec2<i32>>,
        alignment: &Alignment,
        char: char,
        size: i32,
        calc_pos: &Box<fn(i32, &Vec2<i32>, &Alignment) -> Vec2<i32>>,
    ) -> Result<(), Box<dyn Error>> {
        for i in 0..size {
            let ship_pos = calc_pos(i, &start_pos(&self.rect, &alignment), &alignment);
            print_color_at(&ship_pos, char.to_string(), Color::Blue)?;
        }
        Ok(())
    }

    pub fn draw_orientation(&self, orientation: &Alignment) -> Result<(), Box<dyn Error>> {
        let msg = match orientation {
            Alignment::Vertical => "  - Vertical - ",
            Alignment::Horizontal => "  - Horizontal - ",
        };
        let draw_pos = Vec2::new(
            self.rect.pos.x + (self.rect.width / 2) - (msg.len() as i32 / 2),
            self.rect.pos.y + self.rect.height - 2,
        );
        print_color_at(&draw_pos, msg.to_string(), Color::Red)?;
        Ok(())
    }
}

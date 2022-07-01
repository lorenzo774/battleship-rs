// use crate::lib::graphics::print_color_at;
use crate::{
    lib::surface::Surface,
    models::{rect::Rect, space::Vec2},
};
// use crate::models::ship::Ship;
// use crate::models::space::{Alignment, Vec2};
// use crate::settings::SHIP;

use super::widget::{self, Widget};

// use crossterm::cursor::MoveTo;
// use crossterm::execute;
// use crossterm::style::Color;
// use crossterm::terminal::{Clear, ClearType};
// use std::error::Error;
// use std::io::stdout;

use crate::lib::texture::Texture;

/// UI container
pub struct UIContainer {
    pub rect: Rect,
    pub texture: Texture,
    /// A UI container contain widgets
    widgets: Vec<Box<dyn Widget>>,
}
impl UIContainer {
    pub fn new(pos: Vec2<i32>, width: i32, height: i32) -> UIContainer {
        UIContainer {
            texture: Texture::new((width, height)),
            rect: Rect::new(pos, width, height),
            widgets: Vec::new(),
        }
    }

    pub fn insert_widget(&mut self, new_widget: Box<dyn Widget>) {
        self.widgets.push(new_widget);
    }

    pub fn remove_widget(&mut self, widget_index: usize) {
        self.widgets.remove(widget_index);
    }

    pub fn get_surface(&self) -> Surface {
        let mut surface = Surface::new(self.rect.pos.clone(), (self.rect.width, self.rect.height));

        // for widget in &self.widgets {
        //     for (i, px_row) in widget.get_surface().get_texture().iter().enumerate() {
        //         for (j, px) in px_row.iter().enumerate() {
        //             self.surface.texture.change_pixel(
        //                 surf.pos.x + i as i32,
        //                 surf.pos.y + j as i32,
        //                 Some(px.character),
        //                 px.color,
        //             )
        //         }
        //     }
        // }
        // Surface {

        // }
        surface
    }

    // pub fn clear(&self) {
    //     for i in 0..self.rect.height {
    //         execute!(
    //             stdout(),
    //             MoveTo(self.rect.pos.x as u16, (self.rect.pos.y + i) as u16),
    //             Clear(ClearType::UntilNewLine)
    //         )
    //         .unwrap();
    //     }
    // }

    // pub fn draw_msg(&self, y: i32, msg: String) -> Result<(), Box<dyn Error>> {
    //     let draw_pos = Vec2::new(
    //         self.rect.pos.x + (self.rect.width / 2) - (msg.len() as i32 / 2),
    //         self.rect.pos.y + y,
    //     );
    //     print_color_at(&draw_pos, msg.to_string(), Color::White)?;
    //     Ok(())
    // }

    // pub fn draw_selected_ship(
    //     &self,
    //     ship: &Ship,
    //     alignment: &Alignment,
    // ) -> Result<(), Box<dyn Error>> {
    //     let get_start_pos: Box<fn(&Rect, &Alignment) -> Vec2<i32>> =
    //         Box::new(|rect: &Rect, alignment: &Alignment| -> Vec2<i32> {
    //             match alignment {
    //                 Alignment::Vertical => Vec2::new(
    //                     rect.pos.x + (rect.width / 2),
    //                     rect.pos.y + (rect.height / 5) - 1,
    //                 ),
    //                 Alignment::Horizontal => {
    //                     Vec2::new(rect.pos.x + 5, rect.pos.y + (rect.height / 5) + 2)
    //                 }
    //             }
    //         });

    //     let calc_ship_pos: Box<fn(i32, &Vec2<i32>, &Alignment) -> Vec2<i32>> = Box::new(
    //         |i: i32, start_pos: &Vec2<i32>, aligment: &Alignment| -> Vec2<i32> {
    //             match aligment {
    //                 Alignment::Vertical => Vec2::new(start_pos.x, start_pos.y + i + 1),
    //                 Alignment::Horizontal => Vec2::new(start_pos.x + (i * 2) + 1, start_pos.y),
    //             }
    //         },
    //     );
    //     // Clear
    //     self.draw_line(
    //         &get_start_pos,
    //         &Alignment::Horizontal,
    //         ' ',
    //         5,
    //         &calc_ship_pos,
    //     )?;
    //     self.draw_line(
    //         &get_start_pos,
    //         &&Alignment::Vertical,
    //         ' ',
    //         5,
    //         &calc_ship_pos,
    //     )?;
    //     // Draw ship
    //     self.draw_line(&get_start_pos, alignment, SHIP, ship.size(), &calc_ship_pos)?;
    //     Ok(())
    // }

    // fn draw_line(
    //     &self,
    //     start_pos: &Box<fn(&Rect, &Alignment) -> Vec2<i32>>,
    //     alignment: &Alignment,
    //     char: char,
    //     size: i32,
    //     calc_pos: &Box<fn(i32, &Vec2<i32>, &Alignment) -> Vec2<i32>>,
    // ) -> Result<(), Box<dyn Error>> {
    //     for i in 0..size {
    //         let ship_pos = calc_pos(i, &start_pos(&self.rect, &alignment), &alignment);
    //         print_color_at(&ship_pos, char.to_string(), Color::Blue)?;
    //     }
    //     Ok(())
    // }

    // pub fn draw_orientation(&self, orientation: &Alignment) -> Result<(), Box<dyn Error>> {
    //     let msg = match orientation {
    //         Alignment::Vertical => "  - Vertical - ",
    //         Alignment::Horizontal => "  - Horizontal - ",
    //     };
    //     let draw_pos = Vec2::new(
    //         self.rect.pos.x + (self.rect.width / 2) - (msg.len() as i32 / 2),
    //         self.rect.pos.y + self.rect.height - 2,
    //     );
    //     print_color_at(&draw_pos, msg.to_string(), Color::Red)?;
    //     Ok(())
    // }
}

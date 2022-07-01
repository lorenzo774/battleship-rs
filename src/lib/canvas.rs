use std::error::Error;

use crate::{models::space::Vec2, settings::CANVAS_SIZE};

use super::{graphics::print_color_at, surface::Surface};

pub struct Canvas {
    pub surface: Surface,
    pub surfs: Vec<Surface>,
}
impl Canvas {
    pub fn new() -> Canvas {
        Canvas {
            surface: Surface::new(Vec2::zero(), CANVAS_SIZE),
            surfs: Vec::new(),
        }
    }

    /// Algorithm to update the canvas based on the game objects position
    pub fn update_canvas(&mut self) {
        for i in 0..CANVAS_SIZE.0 {
            for j in 0..CANVAS_SIZE.1 {
                self.surface.texture.change_pixel(i, j, Some(' '), None);
            }
        }
        for surf in &self.surfs {
            for (i, px_row) in surf.texture.get_texture().iter().enumerate() {
                for (j, px) in px_row.iter().enumerate() {
                    self.surface.texture.change_pixel(
                        surf.pos.x + i as i32,
                        surf.pos.y + j as i32,
                        Some(px.character),
                        px.color,
                    )
                }
            }
        }
    }

    /// Draw the canvas to the terminal
    pub fn draw_canvas(&self) -> Result<(), Box<dyn Error>> {
        let canvas_pixels = self.surface.texture.get_texture();
        for i in 0..canvas_pixels.len() {
            for j in 0..canvas_pixels[i].len() {
                print_color_at(
                    &Vec2::new(i as i32, j as i32),
                    canvas_pixels[i][j].character.to_string(),
                    canvas_pixels[i][j].color,
                )?;
            }
        }
        Ok(())
    }
}

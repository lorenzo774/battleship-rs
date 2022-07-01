use super::pixel::Pixel;
use crossterm::style::Color;

#[derive(Clone)]
pub struct Texture {
    matrix: Vec<Vec<Pixel>>,
}
impl Texture {
    pub fn new(size: (i32, i32)) -> Texture {
        let mut matrix: Vec<Vec<Pixel>> = Vec::new();
        for i in 0..size.0 {
            matrix.push(Vec::new());
            for _ in 0..size.1 {
                matrix[i as usize].push(Pixel {
                    character: ' ',
                    color: None,
                });
            }
        }
        Texture { matrix }
    }

    /// TODO: Check if the coordinates are valid
    pub fn change_pixel(
        &mut self,
        x: i32,
        y: i32,
        new_char: Option<char>,
        new_color: Option<Color>,
    ) {
        if let Some(n_char) = new_char {
            self.matrix[y as usize][x as usize].character = n_char;
        }
        self.matrix[y as usize][x as usize].color = new_color;
    }

    /// TODO: Check if the coordinates are valid
    pub fn change_m_pixel(
        &mut self,
        x: i32,
        y: i32,
        str: Option<String>,
        new_color: Option<Color>,
    ) {
        if let Some(n_str) = str {
            for (i, c) in n_str.chars().enumerate() {
                self.matrix[y as usize][x as usize + i].character = c;
                if let Some(color) = new_color {
                    self.matrix[y as usize][x as usize + i].color = Some(color);
                }
            }
        }
    }

    pub fn get_texture(&self) -> Vec<Vec<Pixel>> {
        self.matrix.clone()
    }
}

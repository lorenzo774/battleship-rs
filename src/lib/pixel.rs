use crossterm::style::Color;

#[derive(Clone)]
pub struct Pixel {
    pub character: char,
    pub color: Option<Color>,
}

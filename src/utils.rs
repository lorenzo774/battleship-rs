use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::*,
};
use std::io::{stdout, Error};

use crate::models::{rect::Rect, space::Vec2};

pub fn print_color(msg: String, color: Color) -> Result<(), Error> {
    execute!(stdout(), SetForegroundColor(color), Print(msg), ResetColor)?;
    Ok(())
}

pub fn select_char(color: Color, pos: &Vec2<usize>) -> Result<(), Error> {
    execute!(
        stdout(),
        MoveTo(pos.x as u16, pos.y as u16),
        SetBackgroundColor(color),
        Print(" "),
        ResetColor
    )?;
    Ok(())
}

pub fn print_and_clear(msg: String) -> Result<(), Error> {
    execute!(stdout(), Clear(ClearType::CurrentLine), Print(msg))?;
    println!();
    Ok(())
}

pub fn convert_to_rect_pos(pos: &Vec2<usize>, rect: &Rect) -> Option<Vec2<usize>> {
    let table_pos = Vec2::new((pos.x - rect.pos.x) / 2, pos.y - rect.pos.y);
    Some(table_pos)
}

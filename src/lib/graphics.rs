use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::*,
};
use std::io::{stdout, Error};

use crate::models::space::Vec2;

pub fn print_color(msg: String, color: Color) -> Result<(), Error> {
    execute!(stdout(), SetForegroundColor(color), Print(msg), ResetColor)?;
    Ok(())
}

pub fn select_char(color: Color, pos: &Vec2<i32>) -> Result<(), Error> {
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

pub fn print_color_at(pos: &Vec2<i32>, msg: String, color: Color) -> Result<(), Error> {
    execute!(
        stdout(),
        MoveTo(pos.x as u16, pos.y as u16),
        SetForegroundColor(color),
        Print(msg),
        ResetColor
    )?;
    Ok(())
}

pub fn clear_screen() -> Result<(), Error> {
    execute!(stdout(), MoveTo(0, 0), Clear(ClearType::All))?;
    Ok(())
}

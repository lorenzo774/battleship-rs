use std::io::{stdout, Error};

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};

pub fn print_color(msg: String, color: Color) -> Result<(), Error> {
    execute!(stdout(), SetForegroundColor(color), Print(msg), ResetColor)?;
    Ok(())
}

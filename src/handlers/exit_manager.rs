use std::{error::Error, process};

use crossterm::event::KeyCode;

use crate::lib::{
    graphics::{clear_screen, print_and_clear},
    inputs::get_key,
};

pub fn handle_exit() -> Result<(), Box<dyn Error>> {
    if let Some(key_event) = get_key()? {
        match key_event {
            KeyCode::Char('q') | KeyCode::Esc => {
                clear_screen()?;
                print_and_clear("Bye 👋\n".to_string())?;
                process::exit(0)
            }
            _ => (),
        }
    }
    Ok(())
}

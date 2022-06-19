use crate::utils::print_and_clear;
use crossterm::event::{read, Event, KeyCode};

pub fn get_input() -> crossterm::Result<char> {
    const KEYS: [char; 4] = ['h', 'j', 'k', 'l'];

    let event = read()?;
    print_and_clear(format!("Event = {:?}", event))?;

    // Check if the player has enter an exit key
    if event == Event::Key(KeyCode::Esc.into()) || event == Event::Key(KeyCode::Char('q').into()) {
        return Ok('q');
    }
    for i in KEYS {
        if event == Event::Key(KeyCode::Char(i).into()) {
            return Ok(i);
        }
    }
    Ok(' ')
}

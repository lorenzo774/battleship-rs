use crossterm::event::{read, Event, KeyCode};

pub fn get_input() -> crossterm::Result<char> {
    const KEYS: [char; 4] = ['h', 'j', 'k', 'l'];

    let event = read()?;

    // Check if the player has enter an exit key
    if event == Event::Key(KeyCode::Esc.into()) || event == Event::Key(KeyCode::Char('q').into()) {
        return Ok('q');
    }
    if event == Event::Key(KeyCode::Enter.into()) {
        return Ok('E');
    }
    if event == Event::Key(KeyCode::Up.into()) {
        return Ok('V');
    }
    if event == Event::Key(KeyCode::Down.into()) {
        return Ok('H');
    }
    for i in KEYS {
        if event == Event::Key(KeyCode::Char(i).into()) {
            return Ok(i);
        }
    }
    Ok(' ')
}

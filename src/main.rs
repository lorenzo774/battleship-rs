use std::sync::mpsc;
use std::thread;
use std::{error::Error, sync::mpsc::Sender};

mod handlers;
mod lib;
mod models;
mod settings;
mod widgets;

use crossterm::event::{read, Event, KeyCode};
use handlers::game_manager::Game;
use lib::inputs::InputReader;

mod tests;

// Create a new thread for inputs
fn create_input_thread(tx: Sender<Option<KeyCode>>) {
    thread::spawn(move || loop {
        let input = match read().unwrap() {
            Event::Key(key) => Some(key.code),
            _ => None,
        };
        tx.send(input).unwrap();
    });
}

fn main() -> Result<(), Box<dyn Error>> {
    /*
        Create a mpsc channel to allow comunication between threads
        input thread -> main thread
    */
    let (tx, rx) = mpsc::channel();
    create_input_thread(tx);

    let winning = false;
    let input_reader = InputReader::new(rx);
    let mut game = Game::new(input_reader);

    game.start()?;

    // Game loop
    while !winning {
        game.update()?;
    }

    Ok(())
}

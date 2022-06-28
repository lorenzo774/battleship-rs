use crossterm::event::KeyCode;
use std::sync::mpsc::Receiver;

pub struct InputReader {
    rx: Receiver<Option<KeyCode>>,
}
impl InputReader {
    pub fn new(rx: Receiver<Option<KeyCode>>) -> InputReader {
        InputReader { rx }
    }

    pub fn get_key(&self) -> crossterm::Result<Option<KeyCode>> {
        if let Ok(Some(key)) = self.rx.try_recv() {
            return Ok(Some(key));
        }
        Ok(None)
    }
}

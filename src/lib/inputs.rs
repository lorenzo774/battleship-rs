use crossterm::event::KeyCode;
use std::sync::mpsc::Receiver;

// Blocking read
// pub fn get_key() -> crossterm::Result<Option<KeyCode>> {
//     match read()? {
//         Event::Key(key) => Ok(Some(key.code)),
//         Event::Mouse(_) => Ok(None),
//         Event::Resize(_, _) => Ok(None),
//     }
// }

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

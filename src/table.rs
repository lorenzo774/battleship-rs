use crate::config::{ALPHABET, SHIP};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::io::{stdout, Error};

pub struct Table {
    pub size: usize,
    pub matrix: Vec<Vec<char>>,
}
impl Table {
    // Generate a new matrix for the table
    pub fn new(size: usize) -> Table {
        let mut matrix: Vec<Vec<char>> = Vec::new();

        for i in 0..size {
            matrix.push(Vec::new());
            for _ in 0..size {
                matrix[i].push('-');
            }
        }

        Table { size, matrix }
    }

    // Print the table to the console
    pub fn render(&self) -> Result<(), Error> {
        execute!(stdout(), Print("x "))?;
        for k in 0..self.matrix.len() {
            execute!(
                stdout(),
                SetForegroundColor(Color::Blue),
                Print(format!("{} ", k)),
                ResetColor
            )?;
        }
        println!();
        for i in 0..self.matrix.len() {
            execute!(
                stdout(),
                SetForegroundColor(Color::Green),
                Print(format!("{} ", ALPHABET[i])),
                ResetColor
            )?;
            for j in 0..self.matrix[i].len() {
                let color = match self.matrix[i][j] {
                    SHIP => Color::Grey,
                    _ => Color::Cyan,
                };
                execute!(
                    stdout(),
                    SetForegroundColor(color),
                    Print(format!("{} ", self.matrix[i][j]))
                )?;
            }
            println!();
        }
        execute!(stdout(), ResetColor);
        Ok(())
    }
}

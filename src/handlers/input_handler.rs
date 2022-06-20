use crate::config::TABLE_SIZE;
use crate::{lib::inputs::get_input, models::space::Vec2, utils::print_and_clear};
use std::{error::Error, process};

pub fn handle_inputs(select_pos: &mut Vec2<usize>) -> Result<(), Box<dyn Error>> {
    match get_input()? {
        // Exit
        'q' => {
            print_and_clear("Bye 👋\n".to_string())?;
            process::exit(0)
        }
        'E' => print_and_clear("Enter pressed".to_string())?,
        ' ' => print_and_clear("Nothing special is pressed".to_string())?,
        input => {
            print_and_clear("Move inputs".to_string())?;
            match input {
                'j' => select_pos.y += 1,
                'k' => select_pos.y -= 1,
                'h' => select_pos.x -= 2,
                'l' => select_pos.x += 2,
                _ => (),
            };
            if select_pos.x > 2 * TABLE_SIZE {
                select_pos.x = 2 * TABLE_SIZE;
            }
            if select_pos.x < 2 {
                select_pos.x = 2;
            }
            if select_pos.y < 15 {
                select_pos.y = 15;
            }
            if select_pos.y > 14 + TABLE_SIZE {
                select_pos.y = 14 + TABLE_SIZE;
            }
        }
    };
    Ok(())
}

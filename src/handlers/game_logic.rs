//
// This module contains functions to Handle inputs for every state of the game
//

use crate::models::rect::Rect;
use crate::models::ship::ShipType;
use crate::models::space::Alignment;
use crate::models::table::Table;
use crate::{
    lib::{graphics::print_and_clear, inputs::get_input},
    models::space::Vec2,
};
use std::{error::Error, process};

pub fn handle_insertship_inputs(
    table: &mut Table,
    select_pos: &mut Vec2<usize>,
) -> Result<(), Box<dyn Error>> {
    match get_input()? {
        // Exit
        'q' => {
            print_and_clear("Bye 👋\n".to_string())?;
            process::exit(0)
        }
        'E' => handle_insert_new_ship(select_pos, table)?,
        ' ' => print_and_clear("Nothing special is pressed".to_string())?,
        input => handle_movement(&table.rect, select_pos, input)?,
    };
    Ok(())
}

// Handle insert new ship
fn handle_insert_new_ship(
    select_pos: &mut Vec2<usize>,
    table: &mut Table,
) -> Result<(), Box<dyn Error>> {
    let table_pos = match Rect::convert_to_rect_pos(&select_pos, &table.rect) {
        Some(rect_pos) => {
            print_and_clear(format!("Select position = {:?}", rect_pos))?;
            rect_pos
        }
        None => {
            print_and_clear("Can't convert to rect pos".to_string())?;
            Vec2::new(0, 0)
        }
    };

    let res = table.insert_ship(ShipType::Aisle, table_pos, Alignment::Horizontal);
    match res {
        Ok(_) => {}
        Err(msg) => print_and_clear(msg)?,
    }

    Ok(())
}

fn handle_movement(
    table_rect: &Rect,
    select_pos: &mut Vec2<usize>,
    input: char,
) -> Result<(), Box<dyn Error>> {
    print_and_clear("Move inputs".to_string())?;
    match input {
        'j' => select_pos.y += 1,
        'k' => select_pos.y -= 1,
        'h' => select_pos.x -= 2,
        'l' => select_pos.x += 2,
        _ => (),
    };

    table_rect.set_in_boundaries(select_pos);

    Ok(())
}

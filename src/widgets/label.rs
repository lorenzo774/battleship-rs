use crossterm::style::Color;

use crate::{lib::graphics::print_color_at, models::space::Vec2};

use super::{ui_container::UIContainer, widget::Widget};

pub struct Label {
    pos: Vec2<i32>,
    msg: String,
}

impl Label {
    pub fn new(x: i32, y: i32, msg: String) -> Label {
        Label {
            pos: Vec2::new(x, y),
            msg,
        }
    }

    pub fn set_txt(&mut self, msg: String) {
        self.msg = msg;
    }
}

impl Widget for Label {
    fn draw(&self, ui_container: &UIContainer) {
        // Code for title centered
        // let draw_pos = Vec2::new(
        //     ui_container.rect.pos.x + (ui_container.rect.width / 2) - (self.msg.len() as i32 / 2),
        //     ui_container.rect.pos.y + self.y,
        // );
        let draw_pos = Vec2::new(
            ui_container.rect.pos.x + self.pos.x,
            ui_container.rect.pos.y + self.pos.y,
        );
    }
}

use super::ui_container::UIContainer;

pub trait Widget {
    fn draw(&self, ui_container: &UIContainer);
}

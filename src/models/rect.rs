use crate::models::space::Vec2;

pub struct Rect {
    pub pos: Vec2<i32>,
    pub width: i32,
    pub height: i32,
}
impl Rect {
    pub fn new(pos: Vec2<i32>, width: i32, height: i32) -> Rect {
        Rect { pos, width, height }
    }

    pub fn set_in_boundaries(&self, point: &mut Vec2<i32>) {
        if point.x < 0 {
            point.x = 0;
        }
        if point.y < 0 {
            point.y = 0;
        }
        if point.x >= self.width {
            point.x = self.width - 1;
        }
        if point.y >= self.height {
            point.y = self.height - 1;
        }
    }
}

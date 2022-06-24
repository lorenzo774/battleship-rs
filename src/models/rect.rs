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
        if point.x >= self.pos.x + self.width {
            point.x = self.pos.x + self.width - 2;
        }
        if point.x < self.pos.x {
            point.x = self.pos.x;
        }
        if point.y < self.pos.y {
            point.y = self.pos.y;
        }
        if point.y >= self.pos.y + self.height {
            point.y = self.pos.y + self.height - 1;
        }
    }

    pub fn convert_to_rect_pos(pos: &Vec2<i32>, rect: &Rect) -> Option<Vec2<i32>> {
        let table_pos = Vec2::new((pos.x - rect.pos.x) / 2, pos.y - rect.pos.y);
        Some(table_pos)
    }
}

use crate::models::space::Vec2;

pub struct Rect {
    pub pos: Vec2<usize>,
    pub width: usize,
    pub height: usize,
}
impl Rect {
    pub fn new(pos: Vec2<usize>, width: usize, height: usize) -> Rect {
        Rect { pos, width, height }
    }

    pub fn set_in_boundaries(&self, point: &mut Vec2<usize>) {
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

    pub fn convert_to_rect_pos(pos: &Vec2<usize>, rect: &Rect) -> Option<Vec2<usize>> {
        let table_pos = Vec2::new((pos.x - rect.pos.x) / 2, pos.y - rect.pos.y);
        Some(table_pos)
    }
}

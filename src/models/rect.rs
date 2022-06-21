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
}

use crate::models::space::Vec2;

pub struct Boundary {
    pub pos: Vec2<usize>,
    pub size: usize,
}
impl Boundary {
    pub fn new(pos: Vec2<usize>, size: usize) -> Boundary {
        Boundary { pos, size }
    }

    pub fn set_in_boundaries(&self, point: &mut Vec2<usize>) {
        if point.x > self.pos.x + self.size {
            point.x = self.pos.x + self.size;
        }
        if point.x < self.pos.x {
            point.x = self.pos.x;
        }
        if point.y < self.pos.y {
            point.y = self.pos.y;
        }
        if point.y > self.pos.y + self.size {
            point.y = self.pos.y + self.size;
        }
    }
}

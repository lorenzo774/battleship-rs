#[derive(Debug, Clone)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}
impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x, y }
    }
}
impl Vec2<i32> {
    pub fn zero() -> Vec2<i32> {
        Vec2::new(0, 0)
    }
}

#[derive(Debug, Clone)]
pub enum Alignment {
    Vertical,
    Horizontal,
}

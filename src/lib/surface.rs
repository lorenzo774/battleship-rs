use super::texture::Texture;
use crate::models::space::Vec2;

pub struct Surface {
    pub pos: Vec2<i32>,
    pub texture: Texture,
}
impl Surface {
    pub fn new(pos: Vec2<i32>, size: (i32, i32)) -> Surface {
        Surface {
            pos,
            texture: Texture::new(size),
        }
    }
}

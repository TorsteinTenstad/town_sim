use crate::vec2D::*;

#[derive(Copy, Clone)]
pub struct BoundingBox{
    pub pos: Vec2D::<i32>,
    pub size: Vec2D::<i32>,
}
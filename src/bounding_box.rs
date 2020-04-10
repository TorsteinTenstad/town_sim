use crate::vec2D::*;

#[derive(Copy, Clone)]
pub struct BoundingBox{
    pub pos: Vec2D::<f64>,
    pub size: Vec2D::<f64>,
}
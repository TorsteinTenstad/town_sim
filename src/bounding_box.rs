use crate::vec2D::Vec2D;

#[derive(Copy, Clone)]
pub struct BoundingBox {
    pub pos: Vec2D<f64>,
    pub size: Vec2D<f64>,
}

impl BoundingBox {
    pub fn new(x_pos: f64, y_pos: f64, x_size: f64, y_size: f64) -> BoundingBox {
        BoundingBox {
            pos: Vec2D::new(x_pos, y_pos),
            size: Vec2D::new(x_size, y_size),
        }
    }
}

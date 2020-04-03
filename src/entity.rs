use crate::bounding_box::*;

pub enum ShapeType {
    Ellipse,
    Rectangle,
    Triangle,
}

pub struct Entity {
    pub bounding_box: BoundingBox,
    //pub rotation: f64,
    pub color: [f32; 4],
    pub shape_type: ShapeType,
}
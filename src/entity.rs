use crate::bounding_box::BoundingBox;

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

impl Entity {
    pub fn new(bounding_box: BoundingBox, color: [f32; 4], shape_type: ShapeType) -> Entity {
        Entity {
            bounding_box,
            color,
            shape_type,
        }
    }
}

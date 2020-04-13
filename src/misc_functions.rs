use crate::vec2D::Vec2D;

pub fn sum_vecs(points: Vec<Vec2D<f64>>) -> Vec2D<f64> {
    Vec2D::<f64> {
        x: points.iter().map(|point| point.x).sum(),
        y: points.iter().map(|point| point.y).sum(),
    }
}

pub fn get_color_gradient(a: [f32; 4], b: [f32; 4], gradient: f32) -> [f32; 4]{
    let mut c: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
    for i in 0..4{
        c[i] = a[i] + (b[i] - a[i]) * gradient;
    }
    c
}
use crate::vec2D::Vec2D;

pub struct LocationHistory {
    timespan: usize,
    index: usize,
    locations: Vec<Vec2D<f64>>,
}

impl LocationHistory {
    pub fn new(start_pos: Vec2D<f64>, timespan: usize) -> LocationHistory{
        LocationHistory {
            timespan,
            index: 0,
            locations: vec!(start_pos),
        }
    }

    pub fn update(&mut self, new_pos: Vec2D<f64>) -> Vec2D<f64>{
        if self.locations.len() < self.timespan {
            self.locations.push(new_pos);
            self.index += 1;
        } else {
            self.index += 1;
            self.index %= self.timespan;
            self.locations[self.index] = new_pos;
        }
        self.get_mean()
    }

    pub fn get_mean(&self) -> Vec2D<f64>{
        Vec2D::<f64> {
            x: self.locations.iter().map(|location| location.x).sum(),
            y: self.locations.iter().map(|location| location.y).sum(),
        } * (1.0 / self.timespan as f64)
    }

    pub fn get_latest(&self) -> Vec2D<f64>{
        self.locations[self.index]
    }
}

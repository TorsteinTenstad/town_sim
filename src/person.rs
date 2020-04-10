use crate::bounding_box::*;
use crate::entity::*;
use crate::vec2D::*;
use crate::location_history::*;
extern crate rand;
use rand::Rng;

pub struct Person {
    pub entity: Entity,
    pub wander_space: Option<BoundingBox>,
    location_history: LocationHistory,
    speed: f64,
    target_pos: Option<Vec2D<f64>>,
    personal_space_radius: f64,
}

impl Person {
    pub fn new(pos: Vec2D<f64>) -> Person {
        Person {
            entity: Entity {
                bounding_box: BoundingBox {
                    pos,
                    size: Vec2D::<f64> { x: 50.0, y: 50.0 },
                },
                //rotation: 0.5,
                color: [0.125, 0.388, 0.608, 1.0],
                shape_type: ShapeType::Ellipse,
            },
            wander_space: None,
            location_history: LocationHistory::new(pos, 8),
            speed: 50.0,
            target_pos: None,
            personal_space_radius: 70.0,
        }
    }

    pub fn get_latest_pos(&self) -> Vec2D<f64>{
        self.location_history.get_latest()
    }

    pub fn update(
        &mut self,
        dt: f64,
        location_of_closest_other: Option<Vec2D<f64>>,
        distancing_vector: Vec2D<f64>,
    ) {
        let mut delta_pos = distancing_vector * dt * self.speed;
        let reached_target = self.step_towards_target(dt, location_of_closest_other, &mut delta_pos);
        if reached_target || !self.target_pos.is_some() {
            if let Some(wander_space) = self.wander_space {
                let mut rng = rand::thread_rng();
                let rand_vec = Vec2D::<f64> {
                    x: rng.gen::<i64>().abs() as f64,
                    y: rng.gen::<i64>().abs() as f64,
                };
                self.target_pos = Some(
                    (rand_vec % (wander_space.size - self.entity.bounding_box.size))
                        + wander_space.pos,
                );
            }
        }
        self.entity.bounding_box.pos = self.location_history.update(delta_pos);
    }
    
    fn step_towards_target2(&mut self, dt: f64, location_of_closest_other: Option<Vec2D<f64>>,) -> bool{
        if let Some(target_pos) = self.target_pos{
            let diff = target_pos - self.entity.bounding_box.pos;
            let magnitude = diff.magnitude();
            if magnitude < self.speed*dt{
                true
            } else{
                self.entity.bounding_box.pos += diff * (self.speed*dt/magnitude);
                false
            }
        } else{
            false
        }
    }
    fn step_towards_target(
        &mut self,
        dt: f64,
        location_of_closest_other: Option<Vec2D<f64>>,
        delta_pos: &mut Vec2D<f64>,
    ) -> bool {
        let latest_pos = self.location_history.get_latest();
        if let Some(target_pos) = self.target_pos {
            let target_diff = target_pos - latest_pos;
            let target_diff_magnitude = target_diff.magnitude();
            if target_diff_magnitude < self.speed * dt {
                return true;
            } else {
                if let Some(location_of_closest_other) = location_of_closest_other {
                    let closest_other_diff =
                        location_of_closest_other - latest_pos;
                    let closest_other_diff_magnitude = closest_other_diff.magnitude();
                    if closest_other_diff_magnitude < self.personal_space_radius {
                        if target_diff_magnitude < self.personal_space_radius {
                            return true;
                        } else {
                            *delta_pos += closest_other_diff.rotate(90.0)
                                * (self.speed * dt / closest_other_diff_magnitude);
                            return false;
                        }
                    }
                }
                *delta_pos +=
                    target_diff * (self.speed * dt / target_diff_magnitude);
                return false;
            }
        } else {
            return false;
        }
    }
}

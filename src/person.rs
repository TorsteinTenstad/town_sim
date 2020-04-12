use crate::bounding_box::BoundingBox;
use crate::entity::ShapeType;
use crate::entity::Entity;
use crate::vec2D::Vec2D;
use crate::location_history::LocationHistory;
extern crate rand;
use rand::Rng;

pub struct Person {
    pub entity: Entity,
    pub wander_space: Option<BoundingBox>,
    location_history: LocationHistory,
    speed: f64,
    target_pos: Option<Vec2D<f64>>,
    personal_space_radius: f64,
    sick: bool,
    pub sick_risk: f64,
    hygene: f64,
}

impl Person {
    pub fn new(pos: Vec2D<f64>, sick: bool) -> Person {
        let mut color = [0.125, 0.388, 0.608, 1.0];
        let mut sick_risk = 0.0;
        if sick {
            color = [1.0, 0.0, 0.0, 1.0];
            sick_risk = 1.0;
        }
        Person {
            entity: Entity {
                bounding_box: BoundingBox {
                    pos,
                    size: Vec2D::<f64> { x: 50.0, y: 50.0 },
                },
                //rotation: 0.5,
                color,
                shape_type: ShapeType::Ellipse,
            },
            wander_space: None,
            location_history: LocationHistory::new(pos, 8),
            speed: 100.0,
            target_pos: None,
            personal_space_radius: 55.0,
            sick,
            sick_risk,
            hygene: 0.2,
        }
    }

    pub fn get_latest_pos(&self) -> Vec2D<f64>{
        self.location_history.get_latest()
    }

    pub fn update(
        &mut self,
        dt: f64,
        location_of_closest_other: Option<Vec2D<f64>>,
        force: Vec2D<f64>,
        delta_risk: f64,
    ) {
        self.update_health(dt, delta_risk);
        let mut delta_pos = force * dt * self.speed;
        let reached_target = self.get_step_towards_target(dt, location_of_closest_other, &mut delta_pos);
        if reached_target || !self.target_pos.is_some() {
            self.generate_new_target_pos_in_wander_space();
        }
        self.entity.bounding_box.pos = self.location_history.update(delta_pos);
    }

    fn generate_new_target_pos_in_wander_space(&mut self){
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

    fn update_health(&mut self, dt: f64, delta_risk: f64){
        if !self.sick{
            self.sick_risk += delta_risk * dt;
            let mut rng = rand::thread_rng();
            if rng.gen_range(0.0, 1.0) < self.sick_risk * dt{
                self.sick = true;
                self.sick_risk = 1.0;
                self.entity.color = [1.0, 0.0, 0.0, 1.0];
            } else{
                self.sick_risk -= self.hygene * dt;
                if self.sick_risk < 0.0{
                    self.sick_risk = 0.0;
                }
                self.entity.color = [self.sick_risk as f32, 0.388, 0.608, 1.0];
            }
        }
    }

    fn get_step_towards_target(
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

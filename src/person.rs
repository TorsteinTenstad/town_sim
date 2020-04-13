use crate::bounding_box::BoundingBox;
use crate::config;
use crate::entity::Entity;
use crate::entity::ShapeType;
use crate::location_history::LocationHistory;
use crate::misc_functions;
use crate::vec2D::Vec2D;
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
    remaining_sick_time: f64,
    pub sick_risk: f64,
    hygene: f64,
    pub dead: bool,
    immune: bool,
}

impl Person {
    pub fn new(pos: Vec2D<f64>, sick: bool) -> Person {
        let mut color = config::DEFAULT_PERSON_COLOR;
        let mut sick_risk = 0.0;
        let mut remaining_sick_time = 0.0;
        if sick {
            color = config::SICK_COLOR;
            sick_risk = 1.0;
            remaining_sick_time = rand::thread_rng().gen_range(config::VIRUS_MIN_SICK_TIME, config::VIRUS_MAX_SICK_TIME);
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
            remaining_sick_time,
            sick_risk,
            hygene: 0.02,
            dead: false,
            immune: false,
        }
    }

    pub fn get_latest_pos(&self) -> Vec2D<f64> {
        self.location_history.get_latest()
    }

    pub fn update(
        &mut self,
        dt: f64,
        location_of_closest_other: Option<Vec2D<f64>>,
        force: Vec2D<f64>,
        delta_risk: f64,
    ) {
        if !self.dead{
            self.update_health(dt, delta_risk);
            let mut delta_pos = force * dt * self.speed;
            let reached_target =
                self.get_step_towards_target(dt, location_of_closest_other, &mut delta_pos);
            if reached_target || !self.target_pos.is_some() {
                self.generate_new_target_pos_in_wander_space();
            }
            self.entity.bounding_box.pos = self.location_history.update(delta_pos);
        }
    }

    fn generate_new_target_pos_in_wander_space(&mut self) {
        if let Some(wander_space) = self.wander_space {
            let mut rng = rand::thread_rng();
            let rand_vec = Vec2D::<f64> {
                x: rng.gen::<i64>().abs() as f64,
                y: rng.gen::<i64>().abs() as f64,
            };
            self.target_pos = Some(
                (rand_vec % (wander_space.size - self.entity.bounding_box.size)) + wander_space.pos,
            );
        }
    }

    fn update_health(&mut self, dt: f64, delta_risk: f64) {
        let mut rng = rand::thread_rng();
        if !self.sick{
            self.sick_risk += delta_risk * dt;
            if self.sick_risk > 1.0 {
                self.sick_risk = 1.0;
            }
            if !self.immune && rng.gen_range(0.0, 1.0) < self.sick_risk * dt / 10.0 {
                self.sick = true;
                self.sick_risk = 1.0;
                self.entity.color = config::SICK_COLOR;
                self.remaining_sick_time = rng.gen_range(config::VIRUS_MIN_SICK_TIME, config::VIRUS_MAX_SICK_TIME);
            } else {
                self.sick_risk -= self.hygene * dt;
                if self.sick_risk < 0.0 {
                    self.sick_risk = 0.0;
                }
                if !self.immune{
                    self.entity.color = misc_functions::get_color_gradient(
                        config::DEFAULT_PERSON_COLOR,
                        config::MAX_RISK_COLOR,
                        self.sick_risk as f32,
                    );
                }
            }
        } else {
            self.remaining_sick_time -= dt;
            if self.remaining_sick_time < 0.0 {
                self.sick = false;
                if rng.gen_range(0.0, 1.0) < config::VIRUS_FATILITY {
                    self.dead = true;
                    self.entity.color = config::DEAD_COLOR;
                } else {
                    self.immune = true;
                    self.entity.color = config::IMMUNE_COLOR;
                }
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
                    let closest_other_diff = location_of_closest_other - latest_pos;
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
                *delta_pos += target_diff * (self.speed * dt / target_diff_magnitude);
                return false;
            }
        } else {
            return false;
        }
    }
}

use crate::bounding_box::BoundingBox;
use crate::config;
use crate::entity::Entity;
use crate::entity::ShapeType;
use crate::location_history::LocationHistory;
use crate::misc_functions;
use crate::vec2D::Vec2D;
use crate::schedule_entry::ScheduleEntry;
use crate::schedule::Schedule;
extern crate rand;
use rand::Rng;

pub struct Person {
    pub entity: Entity,
    pub wander_space: Option<BoundingBox>,
    location_history: LocationHistory,
    speed: f64,
    target_pos: Vec<Vec2D<f64>>,
    personal_space_radius: f64,
    sick: bool,
    remaining_sick_time: f64,
    pub sick_risk: f64,
    hygene: f64,
    pub dead: bool,
    immune: bool,
    pub schedule: Schedule,
}

impl Person {
    pub fn new(pos: Vec2D<f64>, sick: bool) -> Person {
        let mut rng = rand::thread_rng();
        let mut color = config::DEFAULT_PERSON_COLOR;
        let mut sick_risk = 0.0;
        let mut remaining_sick_time = 0.0;
        if sick {
            color = config::SICK_COLOR;
            sick_risk = 1.0;
            remaining_sick_time = rng.gen_range(config::VIRUS_MIN_SICK_TIME, config::VIRUS_MAX_SICK_TIME);
        }
        let mut schedule = Schedule::new();
        for x in vec![0.0, 8.0, 16.0]{
            let id: usize = rng.gen_range(0, 4);
            schedule.push(ScheduleEntry::new(x, id));
        }
        Person {
            entity: Entity::new(
                BoundingBox::new(pos.x, pos.y, 50.0, 50.0),
                color,
                ShapeType::Ellipse,
            ),
            wander_space: None,
            location_history: LocationHistory::new(pos, 8),
            speed: 500.0,
            target_pos: Vec::new(),
            personal_space_radius: 55.0,
            sick,
            remaining_sick_time,
            sick_risk,
            hygene: 0.02,
            dead: false,
            immune: false,
            schedule,
        }
    }

    pub fn get_latest_pos(&self) -> Vec2D<f64> {
        self.location_history.get_latest()
    }

    pub fn update(
        &mut self,
        dt: f64,
        time: f64,
        location_of_closest_other: Option<Vec2D<f64>>,
        force: Vec2D<f64>,
        delta_risk: f64,
    ) {
        if !self.dead {
            self.update_health(dt, delta_risk);
            if self.target_pos.len() == 0 {
                self.generate_new_target_pos_in_wander_space();
            }
            let mut new_pos = self.get_latest_pos()
                + self.get_step_towards_target(dt, location_of_closest_other)
                + force * dt * self.speed;
            //self.keep_pos_inside_wander_space(&mut new_pos);
            self.entity.bounding_box.pos = self.location_history.update(new_pos);
        }
    }

    fn keep_pos_inside_wander_space(&self, pos: &mut Vec2D<f64>) {
        if let Some(wander_space) = self.wander_space{
            if pos.x < wander_space.pos.x {
                pos.x = wander_space.pos.x;
            } else if pos.x
                > wander_space.pos.x + wander_space.size.x - self.entity.bounding_box.size.x
            {
                pos.x = wander_space.pos.x + wander_space.size.x - self.entity.bounding_box.size.x;
            }
            if pos.y < wander_space.pos.y {
                pos.y = wander_space.pos.y;
            } else if pos.y
                > wander_space.pos.y + wander_space.size.y - self.entity.bounding_box.size.y
            {
                pos.y = wander_space.pos.y + wander_space.size.y - self.entity.bounding_box.size.y;
            }
        }
    }

    fn generate_new_target_pos_in_wander_space(&mut self) {
        if let Some(wander_space) = self.wander_space {
            let mut rng = rand::thread_rng();
            self.target_pos.push(Vec2D::<f64>::new(
                rng.gen_range(
                    wander_space.pos.x,
                    wander_space.pos.x + wander_space.size.x - self.entity.bounding_box.size.x,
                ),
                rng.gen_range(
                    wander_space.pos.y,
                    wander_space.pos.y + wander_space.size.y - self.entity.bounding_box.size.x,
                ),
            ));
        }
    }

    fn update_health(&mut self, dt: f64, delta_risk: f64) {
        let mut rng = rand::thread_rng();
        if !self.sick {
            self.sick_risk += delta_risk * dt;
            if self.sick_risk > 1.0 {
                self.sick_risk = 1.0;
            }
            if !self.immune && rng.gen_range(0.0, 1.0) < self.sick_risk * dt / 10.0 {
                self.sick = true;
                self.sick_risk = 1.0;
                self.entity.color = config::SICK_COLOR;
                self.remaining_sick_time =
                    rng.gen_range(config::VIRUS_MIN_SICK_TIME, config::VIRUS_MAX_SICK_TIME);
            } else {
                self.sick_risk -= self.hygene * dt;
                if self.sick_risk < 0.0 {
                    self.sick_risk = 0.0;
                }
                if !self.immune {
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
    ) -> Vec2D<f64> {
        let latest_pos = self.location_history.get_latest();
        if self.target_pos.len() != 0 {
            let target_diff = self.target_pos[0] - latest_pos;
            let target_diff_magnitude = target_diff.magnitude();
            if target_diff_magnitude < self.speed * dt {
                self.target_pos.remove(0);
                return target_diff;
            } else {
                if let Some(location_of_closest_other) = location_of_closest_other {
                    let closest_other_diff = location_of_closest_other - latest_pos;
                    let closest_other_diff_magnitude = closest_other_diff.magnitude();
                    if closest_other_diff_magnitude < self.personal_space_radius {
                        if target_diff_magnitude < self.personal_space_radius {
                            self.target_pos.remove(0);
                            return Vec2D::<f64>::new(0.0, 0.0);
                        } else {
                            return closest_other_diff.rotate(90.0)
                                * (self.speed * dt / closest_other_diff_magnitude);
                        }
                    }
                }
                return target_diff * (self.speed * dt / target_diff_magnitude);
            }
        } else {
            return Vec2D::<f64>::new(0.0, 0.0);
        }
    }
}

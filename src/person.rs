extern crate rand;
use rand::Rng;
use crate::entity::*;
use crate::bounding_box::*;
use crate::vec2D::*;

pub struct Person{
    pub entity: Entity,
    pub wander_space: Option<BoundingBox>,
    speed: f64,
    target_pos: Option<Vec2D<i32>>,
    personal_space_radius: f64,
}

impl Person{
    pub fn new(pos: Vec2D::<i32>) -> Person{
        Person{
            entity: Entity {
                bounding_box: BoundingBox{
                    pos,
                    size: Vec2D::<i32>{x: 50, y: 50},
                },
                //rotation: 0.5,
                color: [0.125, 0.388, 0.608, 1.0],
                shape_type: ShapeType::Ellipse,
            },
            speed: 200.0,
            wander_space: None,
            target_pos: None,
            personal_space_radius: 50.0,
        }
    }

    pub fn update(&mut self, dt: f64, location_of_closest_other: Option<Vec2D::<i32>>){
        let reached_target = self.step_towards_target(dt, location_of_closest_other);
        if reached_target || !self.target_pos.is_some(){
            if let Some(wander_space) = self.wander_space{
                let mut rng = rand::thread_rng();
                let rand_vec = Vec2D::<i32>{x: rng.gen::<i32>().abs(), y: rng.gen::<i32>().abs(), };
                self.target_pos = Some((rand_vec % (wander_space.size - self.entity.bounding_box.size)) + wander_space.pos);
            }
        }
    }

    fn step_towards_target(&mut self, dt: f64, location_of_closest_other: Option<Vec2D::<i32>>) -> bool{
        if let Some(target_pos) = self.target_pos{
            let target_diff = target_pos - self.entity.bounding_box.pos;
            let target_diff_magnitude = target_diff.magnitude();
            if target_diff_magnitude < self.speed*dt{
                true
            } else{
                if let Some(location_of_closest_other) = location_of_closest_other{
                    let closest_other_diff = location_of_closest_other - self.entity.bounding_box.pos;
                    let closest_other_diff_magnitude = closest_other_diff.magnitude();
                    if closest_other_diff_magnitude < self.personal_space_radius{
                        if target_diff_magnitude < self.personal_space_radius{
                            true
                        } else{
                            self.entity.bounding_box.pos += (self.speed*dt/closest_other_diff_magnitude) * closest_other_diff.rotate(90.0);
                            false
                        }
                    } else{
                        self.entity.bounding_box.pos += (self.speed*dt/target_diff_magnitude) * target_diff;
                        false
                    }
                } else{
                    self.entity.bounding_box.pos += (self.speed*dt/target_diff_magnitude) * target_diff;
                    false
                }
            }
        } else{
            false
        }
    }
}
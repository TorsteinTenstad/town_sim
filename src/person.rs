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
}

impl Person{
    pub fn new() -> Person{
        Person{
            entity: Entity {
                bounding_box: BoundingBox{
                    pos: Vec2D::<i32>{x: 0, y: 0},
                    size: Vec2D::<i32>{x: 50, y: 50},
                },
                //rotation: 0.5,
                color: [0.125, 0.388, 0.608, 1.0],
                shape_type: ShapeType::Ellipse,
            },
            speed: 200.0,
            wander_space: None,
            target_pos: None,
        }
    }

    pub fn update(&mut self, dt: f64){
        let reached_target = self.step_towards_target(dt);
        if reached_target || !self.target_pos.is_some(){
            if let Some(wander_space) = self.wander_space{
                let mut rng = rand::thread_rng();
                let rand_vec = Vec2D::<i32>{x: rng.gen::<i32>().abs(), y: rng.gen::<i32>().abs(), };
                self.target_pos = Some((rand_vec % (wander_space.size - self.entity.bounding_box.size)) + wander_space.pos);
            }
        }
    }

    fn step_towards_target(&mut self, dt: f64) -> bool{
        if let Some(target_pos) = self.target_pos{
            let diff = target_pos - self.entity.bounding_box.pos;
            let magnitude = diff.magnitude();
            if magnitude < self.speed*dt{
                true
            } else{
                self.entity.bounding_box.pos += (self.speed*dt/magnitude) * diff;
                false
            }
        } else{
            false
        }
    }
}
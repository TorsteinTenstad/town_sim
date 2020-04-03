extern crate rand;
use rand::Rng;
use crate::bounding_box::*;
use crate::entity::*;

pub struct Person{
    pub entity: Entity,
    pub wander_space: Option<BoundingBox>,
    speed: f64,
    target_pos: Option<[i32; 2]>,
}

impl Person{
    pub fn new() -> Person{
        Person{
            entity: Entity {
                bounding_box: BoundingBox{
                    pos: [0, 0],
                    size: [50, 50],
                },
                //rotation: 0.5,
                color: [0.125, 0.388, 0.608, 1.0],
                shape_type: ShapeType::Ellipse,
            },
            speed: 200.0,
            wander_space: None,
            target_pos: Some([50, 50]),
        }
    }

    pub fn update(&mut self, dt: f64){
        self.step_wander(dt);
    }

    fn step_wander(&mut self, dt: f64){
        let reached_target = self.step_towards_target(dt);
        if reached_target{
            if let Some(wander_space) = self.wander_space{
                let mut rng = rand::thread_rng();
                self.target_pos = Some([rng.gen::<i32>().abs() % (wander_space.size[0] - self.entity.bounding_box.size[0]) + wander_space.pos[0], rng.gen::<i32>().abs() % (wander_space.size[1] - self.entity.bounding_box.size[1]) + wander_space.pos[1]]);
            }
        }
    }

    fn step_towards_target(&mut self, dt: f64) -> bool{
        if let Some(target_pos) = self.target_pos{
            let diff = [(target_pos[0] - self.entity.bounding_box.pos[0]) as f64, (target_pos[1] - self.entity.bounding_box.pos[1]) as f64];
            let distance = (diff[0]*diff[0]+diff[1]*diff[1]).abs().sqrt();
            if distance < self.speed*dt{
                true
            } else{
                self.entity.bounding_box.pos[0] += (self.speed*dt*diff[0]/distance) as i32;
                self.entity.bounding_box.pos[1] += (self.speed*dt*diff[1]/distance) as i32;
                false
            }
        } else{
            false
        }
    }
}
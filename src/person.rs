extern crate rand;
use rand::Rng;
use crate::bounding_box::*;
use crate::entity::*;

pub struct Person{
    pub entity: Entity,
    pub wander_space: Option<BoundingBox>,
    speed: f64,
    target_x: i32,
    target_y: i32,
}

impl Person{
    pub fn new() -> Person{
        Person{
            entity: Entity {
                bounding_box: BoundingBox{
                    x_pos: 0,
                    y_pos: 0,
                    x_size: 50,
                    y_size: 50,
                },
                //rotation: 0.5,
                color: [0.125, 0.388, 0.608, 1.0],
                shape_type: ShapeType::Ellipse,
            },
            speed: 200.0,
            wander_space: None,
            target_x: 50,
            target_y: 50,
        }
    }

    pub fn update(&mut self, dt: f64){
        let reached_target = self.step_towards_target(dt);
        if reached_target{
            if let Some(space) = self.wander_space{
            let mut rng = rand::thread_rng();
                self.target_x = rng.gen::<i32>().abs() % (space.x_size - self.entity.bounding_box.x_size) + space.x_pos ;
                self.target_y = rng.gen::<i32>().abs() % (space.y_size - self.entity.bounding_box.y_size) + space.y_pos ;
            }
        }
    }

    fn step_towards_target(&mut self, dt: f64) -> bool{
        let x_diff = (self.target_x - self.entity.bounding_box.x_pos) as f64;
        let y_diff = (self.target_y - self.entity.bounding_box.y_pos) as f64;
        let distance = (x_diff*x_diff+y_diff*y_diff).abs().sqrt();
        if distance < self.speed*dt{
            true
        } else{
            self.entity.bounding_box.x_pos += (self.speed*dt*x_diff/distance) as i32;
            self.entity.bounding_box.y_pos += (self.speed*dt*y_diff/distance) as i32;
            false
        }
    }
}
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
                self.target_pos = Some(Vec2D::<i32>{x: rng.gen::<i32>().abs() % (wander_space.size.x - self.entity.bounding_box.size.x) + wander_space.pos.x, y: rng.gen::<i32>().abs() % (wander_space.size.y - self.entity.bounding_box.size.y) + wander_space.pos.y});
            }
        }
    }

    fn step_towards_target(&mut self, dt: f64) -> bool{
        if let Some(target_pos) = self.target_pos{
            let diff = Vec2D::<f64>{x: (target_pos.x - self.entity.bounding_box.pos.x) as f64, y: (target_pos.y - self.entity.bounding_box.pos.y) as f64};
            let distance = (diff.x*diff.x+diff.y*diff.y).abs().sqrt();
            if distance < self.speed*dt{
                true
            } else{
                self.entity.bounding_box.pos.x += (self.speed*dt*diff.x/distance) as i32;
                self.entity.bounding_box.pos.y += (self.speed*dt*diff.y/distance) as i32;
                false
            }
        } else{
            false
        }
    }
}
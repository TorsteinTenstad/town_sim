use crate::bounding_box::*;
use crate::building::*;
use crate::entity::*;
use crate::person::Person;
use crate::vec2D::*;
extern crate rand;
use rand::Rng;

pub struct Town {
    pub people: Vec<Person>,
    pub buildings: Vec<Building>,
}

impl Town {
    pub fn new() -> Town {
        let mut town = Town {
            people: Vec::<Person>::new(),
            buildings: Vec::<Building>::new(),
        };
        let mut building = Building {
            entity: Entity {
                bounding_box: BoundingBox {
                    pos: Vec2D::<f64> { x: 50.0, y: 50.0 },
                    size: Vec2D::<f64> { x: 1340.0 , y: 1340.0 },
                },
                color: [0.5, 0.5, 0.5, 1.0],
                shape_type: ShapeType::Rectangle,
            },
        };
        let mut rng = rand::thread_rng();
        let spread = 700.0;
        for i in 0..50 {
            let mut person = Person::new(Vec2D::<f64> {
                x: rng.gen_range(720.0 - spread, 720.0 + spread),
                y: rng.gen_range(720.0 - spread, 720.0 + spread),
            });
            person.wander_space = Some(building.entity.bounding_box);
            town.people.push(person);
        }
        town.buildings.push(building);
        town
    }

    pub fn update(&mut self, dt: f64) {
        let people_points: Vec<Vec2D<f64>> = self
            .people
            .iter()
            .map(|person| person.get_latest_pos())
            .collect();
        let closest_points: Vec<Option<Vec2D<f64>>> = people_points
            .iter()
            .map(|pos| self.get_closest_person(&pos))
            .collect();
        let distancing_vectors: Vec<Vec2D<f64>> = people_points
            .iter()
            .map(|pos| self.get_distancing_vectors(&pos))
            .collect();

        for (person, (closest_point, distancing_vector)) in self
            .people
            .iter_mut()
            .zip(closest_points.iter().zip(distancing_vectors.iter()))
        {
            person.update(dt, *closest_point, *distancing_vector);
        }
    }

    fn get_closest_person(&mut self, origin: &Vec2D<f64>) -> Option<Vec2D<f64>> {
        self.people
            .iter()
            .map(|person| person.get_latest_pos())
            .filter(|pos| pos != origin)
            .min_by_key(|pos| (1000.0 * (*origin - *pos).magnitude()) as u64)
    }

    fn get_distancing_vectors(&mut self, origin: &Vec2D<f64>) -> Vec2D<f64> {
        let mut vec = sum_vecs(
            self.people
                .iter()
                .map(|person| person.get_latest_pos())
                .filter(|pos| pos != origin)
                .map(|pos| (pos, (*origin - pos).magnitude()))
                .filter(|(pos, r)| *r < 200.0)
                .map(|(pos, r)| {
                    let a = ((origin.y - pos.y) as f64).atan2((origin.x - pos.x) as f64);
                    Vec2D::<f64>::new(a.cos(), a.sin()) * (2500000000.0 / r.powi(4))
                })
                .collect(),
        );
        let magnitude = vec.magnitude();
        if magnitude > 0.5{
            vec = vec * (0.5/magnitude);
        }
        vec
    }
}

pub fn sum_vecs(points: Vec<Vec2D<f64>>) -> Vec2D<f64> {
    Vec2D::<f64> {
        x: points.iter().map(|point| point.x).sum(),
        y: points.iter().map(|point| point.y).sum(),
    }
}

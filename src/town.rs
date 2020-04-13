use crate::bounding_box::BoundingBox;
use crate::building::Building;
use crate::config;
use crate::entity::ShapeType;
use crate::entity::Entity;
use crate::person::Person;
use crate::vec2D::Vec2D;
use crate::misc_functions;
extern crate rand;
use rand::Rng;

pub struct Town {
    pub people: Vec<Person>,
    pub buildings: Vec<Building>,
}

impl Town {
    pub fn new() -> Town {
        let mut town = Town {
            people: Vec::new(),
            buildings: Vec::new(),
        };
        let building = Building::new(Entity::new(
            BoundingBox::new(50.0, 50.0, 1340.0, 1340.0),
            config::DEFAULT_BUILDING_COLOR,
            ShapeType::Rectangle,
        ));
        let mut rng = rand::thread_rng();
        for i in 0..50 {
            let mut person = Person::new(
                Vec2D::<f64>::new(
                    rng.gen_range(50.0, 1340.0),
                    rng.gen_range(50.0, 1340.0),
                ),
                i == 0,
            );
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
        let forces: Vec<Vec2D<f64>> = people_points
            .iter()
            .map(|pos| self.get_force(&pos))
            .collect();
        let delta_sick_risks: Vec<f64> = people_points
            .iter()
            .map(|pos| self.get_delta_sick_risk(&pos))
            .collect();

        for (person, (closest_point, (force, delta_sick_risk))) in self.people.iter_mut().zip(
            closest_points
                .iter()
                .zip(forces.iter().zip(delta_sick_risks.iter())),
        ) {
            person.update(dt, *closest_point, *force, *delta_sick_risk);
        }
    }

    fn get_closest_person(&mut self, origin: &Vec2D<f64>) -> Option<Vec2D<f64>> {
        self.people
            .iter()
            .filter(|person| !person.dead)
            .map(|person| person.get_latest_pos())
            .filter(|pos| pos != origin)
            .min_by_key(|pos| (1000.0 * (*origin - *pos).magnitude()) as u64)
    }

    fn get_force(&mut self, origin: &Vec2D<f64>) -> Vec2D<f64> {
        let mut vec = misc_functions::sum_vecs(
            self.people
                .iter()
                .filter(|person| !person.dead)
                .map(|person| person.get_latest_pos())
                .filter(|pos| pos != origin)
                .map(|pos| (pos, (*origin - pos).magnitude()))
                .filter(|(_pos, r)| *r < config::FORCE_REACH)
                .map(|(pos, r)| {
                    let a = ((origin.y - pos.y) as f64).atan2((origin.x - pos.x) as f64);
                    Vec2D::<f64>::new(a.cos(), a.sin()) * (config::FORCE_STRENGTH / r.powi(config::FORCE_POW))
                })
                .collect(),
        );
        let magnitude = vec.magnitude();
        if magnitude > config::MAX_FORCE {
            vec = vec * (config::MAX_FORCE / magnitude);
        }
        vec
    }

    fn get_delta_sick_risk(&mut self, origin: &Vec2D<f64>) -> f64 {
        let vec: Vec<f64> = self
            .people
            .iter()
            .filter(|person| !person.dead)
            .map(|person| (person.get_latest_pos(), person.sick_risk))
            .filter(|(pos, _sick_risk)| pos != origin)
            .map(|(pos, sick_risk)| ((*origin - pos).magnitude(), sick_risk))
            .filter(|(r, _sick_risk)| *r < config::VIRUS_RISK_REACH)
            .map(|(r, sick_risk)| sick_risk * (config::VIRUS_RISK_STRENGTH / r.powi(config::VIRUS_RISK_POW)))
            .collect();
        let mut sum = 0.0;
        for x in vec {
            sum += x;
        }
        if sum > config::MAX_VIRUS_RISK {
            sum = config::MAX_VIRUS_RISK;
        }
        sum
    }
}

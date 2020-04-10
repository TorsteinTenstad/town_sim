use crate::vec2D::*;
use crate::bounding_box::*;
use crate::entity::*;
use crate::person::*;
use crate::building::*;

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
        let mut building = Building{
            entity: Entity{
                bounding_box: BoundingBox{
                    pos: Vec2D::<i32>{x: 50, y: 50},
                    size: Vec2D::<i32>{x: 300, y: 300},
                },
            color: [0.5, 0.5, 0.5, 1.0],
            shape_type: ShapeType::Rectangle
            },
        };
        for i in 0..2{
            let mut person = Person::new(Vec2D::<i32>{x: i*50, y: i*50});
            person.wander_space = Some(building.entity.bounding_box);
            town.people.push(person);
        }
        town.buildings.push(building);
        town
    }

    pub fn update(&mut self, dt: f64){
        let people_points: Vec<Vec2D::<i32>> = self.people.iter().map(|person| person.entity.bounding_box.pos).collect();
        let closest_points: Vec<Option<Vec2D::<i32>>> =people_points.iter().map(|pos| self.get_location_of_closest_person(*pos)).collect();
    
        for (person, closest_point) in self.people.iter_mut().zip(closest_points.iter()) {
            person.update(dt, *closest_point);
        }
    }
    
    fn get_location_of_closest_person(&mut self, pos: Vec2D::<i32>) -> Option<Vec2D::<i32>>{
        self.people.iter().filter(|person|
            person.entity.bounding_box.pos != pos
        ).min_by_key(|person|
            (pos.x - person.entity.bounding_box.pos.x).abs() + (pos.y - person.entity.bounding_box.pos.y).abs()
        ).map(|person|
            person.entity.bounding_box.pos
        )
    }
}
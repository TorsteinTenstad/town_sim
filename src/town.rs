use crate::entity::*;
use crate::person::*;
use crate::bounding_box::*;

pub struct Town {
    pub people: Vec<Person>,
    pub buildings: Vec<Entity>,
}

impl Town {
    pub fn new() -> Town {
        let mut town = Town {
            people: Vec::<Person>::new(),
            buildings: Vec::<Entity>::new(),
        };
        let mut building = Entity{
            bounding_box: BoundingBox{
                x_pos: 50,
                y_pos: 50,
                x_size: 200,
                y_size: 200,
            },
            color: [0.5, 0.5, 0.5, 1.0],
            shape_type: ShapeType::Rectangle
        };
        let mut person = Person::new();
        person.wander_space = Some(building.bounding_box);
        town.people.push(person);
        town.buildings.push(building);
        town
    }
}
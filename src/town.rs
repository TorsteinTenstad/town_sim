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
                    size: Vec2D::<i32>{x: 200, y: 200},
                },
            color: [0.5, 0.5, 0.5, 1.0],
            shape_type: ShapeType::Rectangle
            },
        };
        let mut person = Person::new();
        person.wander_space = Some(building.entity.bounding_box);
        town.people.push(person);
        town.buildings.push(building);
        town
    }
}
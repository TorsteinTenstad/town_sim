use crate::entity::Entity;

pub struct Building {
    pub entity: Entity,
}

impl Building {
    pub fn new(entity: Entity) -> Building {
        Building { entity }
    }
}

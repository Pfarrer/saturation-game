use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Connection {
    pub between: (Entity, Entity),
}

impl Connection {
    pub fn connects_to(&self, entity: Entity) -> bool {
        self.between.0 == entity || self.between.1 == entity
    }
}

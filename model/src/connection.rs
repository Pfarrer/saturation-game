use bevy::prelude::*;

/// Between values will be orded the same way, no matter in which order they are
/// provided upon create:
/// ```
/// # use bevy::ecs::entity::Entity;
/// # use model::connection::Connection;
/// let a = Entity::from_raw(1);
/// let b = Entity::from_raw(2);
/// let connection_a_b = Connection::new_between(a, b);
/// let connection_b_a = Connection::new_between(b, a);
/// assert_eq!(connection_a_b.between(), connection_b_a.between());
/// ```
#[derive(Component, Debug, Clone)]
pub struct Connection {
    between: (Entity, Entity),
    pub energy_flow: f64,
}

impl Connection {
    /// Prevents a connection between the same entity
    /// ```should_panic
    /// # use bevy::ecs::entity::Entity;
    /// # use model::connection::Connection;
    /// let a = Entity::from_raw(1);
    /// Connection::new_between(a, a);
    /// ```
    pub fn new_between(a: Entity, b: Entity) -> Connection {
        if a == b {
            panic!("Connections between equal entities are not allowed");
        }

        let min_entity = std::cmp::min(a, b);
        let max_entity = std::cmp::max(a, b);

        Connection {
            between: (min_entity, max_entity),
            energy_flow: 0.,
        }
    }

    pub fn between(&self) -> &(Entity, Entity) {
        &self.between
    }

    pub fn connects_to(&self, entity: Entity) -> bool {
        self.between.0 == entity || self.between.1 == entity
    }
}

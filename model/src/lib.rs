use bevy::prelude::{Entity, App, Plugin};
use connection::Connection;
use construction::Construction;

pub mod connection;
pub mod construction;
pub mod resources;

#[derive(Debug)]
pub struct RemovalEvent<T> {
    pub entity: Entity,
    pub component: T,
}

pub struct ModelPlugin;

impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RemovalEvent<Construction>>()
            .add_event::<RemovalEvent<Connection>>();
    }
}

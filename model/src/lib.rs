use bevy::prelude::{App, Entity, Plugin};
use collision::CollisionEvent;
use connection::Connection;
use construction::Construction;
use game::{GameEvent, GameMode};

pub mod collision;
pub mod connection;
pub mod construction;
pub mod game;
pub mod game_settings;
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
            .add_event::<RemovalEvent<Connection>>()
            .add_event::<GameEvent>()
            .add_event::<CollisionEvent>()
            .insert_resource(game_settings::GameSettings)
            .insert_resource(GameMode::Idle)
            .add_system(game::game_mode_debug_system);
    }
}

use bevy::prelude::*;

mod connections;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(connections::on_construction_remove_system);
    }
}

use bevy::prelude::*;

mod connections;
mod game_mode;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(connections::on_construction_remove_system)
            .add_system(game_mode::switch_game_mode_system);
    }
}

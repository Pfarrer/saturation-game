use bevy::asset::AssetServerSettings;
use bevy::prelude::*;
use game::GamePlugin;
use model::construction::{Construction, ConstructionKind, ConstructionStatus};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(AssetServerSettings {
            asset_folder: "../../assets".to_string(),
            watch_for_changes: false,
        })
        .insert_resource(WindowDescriptor {
            title: "Saturation".to_string(),
            width: 800.,
            height: 600.,
            present_mode: bevy::window::PresentMode::Fifo,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_startup_system(init_game_system)
        .run();
}

fn init_game_system(mut commands: Commands) {
    commands.spawn().insert(Construction {
        location: Vec2::new(0., 0.),
        kind: ConstructionKind::Base,
        status: ConstructionStatus::Operating,
        influence_radius: 70.,
    });
}

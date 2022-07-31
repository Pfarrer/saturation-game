use bevy::asset::AssetServerSettings;
use bevy::prelude::*;
use connection::ConnectionShapePlugin;
use construction::ConstructionShapePlugin;
use game::GamePlugin;
use hud::HudPlugin;
use influence::InfluenceShapePlugin;
use model::{
    construction::{Construction, ConstructionKind},
    ModelPlugin,
};
use resources::ResourcesPlugin;

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
        .add_plugin(ModelPlugin)
        .add_plugin(GamePlugin)
        .add_startup_system(init_camera_system)
        .add_plugin(ResourcesPlugin)
        .add_plugin(ConstructionShapePlugin)
        .add_plugin(InfluenceShapePlugin)
        .add_plugin(ConnectionShapePlugin)
        .add_plugin(HudPlugin)
        .add_startup_system(init_game_system)
        .run();
}

fn init_camera_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn init_game_system(mut commands: Commands) {
    commands.spawn().insert(Construction {
        location: Vec2::new(0., 0.),
        kind: ConstructionKind::Base,
        influence_radius: 70.,
    });
}
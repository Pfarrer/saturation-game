use bevy::asset::AssetServerSettings;
use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use game::GamePlugin;
use model::connection::Connection;
use model::construction::{Construction, ConstructionKind, ConstructionStatus};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(AssetServerSettings {
            asset_folder: "../../assets".to_string(),
            watch_for_changes: false,
        })
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_startup_system(init_game_system)
        .run();
}

fn init_game_system(mut commands: Commands) {
    let base_construction_entity = commands
        .spawn()
        .insert(Construction {
            location: Vec2::new(0., 0.),
            kind: ConstructionKind::Base,
            status: ConstructionStatus::Operating,
        })
        .id();

    let collector1_construction_entity = commands
        .spawn()
        .insert(Construction {
            location: Vec2::new(-100., 50.),
            kind: ConstructionKind::Collector,
            status: ConstructionStatus::Operating,
        })
        .id();
    commands.spawn().insert(Connection::new_between(
        base_construction_entity.clone(),
        collector1_construction_entity.clone(),
    ));

    let collector2_construction_entity = commands
        .spawn()
        .insert(Construction {
            location: Vec2::new(-100., -50.),
            kind: ConstructionKind::Collector,
            status: ConstructionStatus::Operating,
        })
        .id();
    commands.spawn().insert(Connection::new_between(
        base_construction_entity.clone(),
        collector2_construction_entity.clone(),
    ));
    commands.spawn().insert(Connection::new_between(
        collector1_construction_entity.clone(),
        collector2_construction_entity.clone(),
    ));

    let extractor1_construction_entity = commands
        .spawn()
        .insert(Construction {
            location: Vec2::new(100., 0.),
            kind: ConstructionKind::Extractor,
            status: ConstructionStatus::Operating,
        })
        .id();
    commands.spawn().insert(Connection::new_between(
        base_construction_entity.clone(),
        extractor1_construction_entity.clone(),
    ));

    let extractor2_construction_entity = commands
        .spawn()
        .insert(Construction {
            location: Vec2::new(200., 0.),
            kind: ConstructionKind::Extractor,
            status: ConstructionStatus::Operating,
        })
        .id();
    commands.spawn().insert(Connection::new_between(
        extractor2_construction_entity.clone(),
        extractor1_construction_entity.clone(),
    ));
}

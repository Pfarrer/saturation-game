use bevy::asset::AssetServerSettings;
use bevy::prelude::*;
use game::GamePlugin;
use model::{
    connection::Connection,
    construction::{Construction, ConstructionKind, ConstructionStatus},
};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(AssetServerSettings {
            asset_folder: "../../assets".to_string(),
            watch_for_changes: false,
        })
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
            influence_radius: 70.,
        })
        .id();

    let collector1_construction_entity = commands
        .spawn()
        .insert(Construction {
            location: Vec2::new(0., 125.),
            kind: ConstructionKind::Collector,
            status: ConstructionStatus::Operating,
            influence_radius: 50.,
        })
        .id();
    commands.spawn().insert(Connection {
        between: (
            base_construction_entity.clone(),
            collector1_construction_entity.clone(),
        ),
    });

    let collector2_construction_entity = commands
        .spawn()
        .insert(Construction {
            location: Vec2::new(-100., 175.),
            kind: ConstructionKind::Extractor,
            status: ConstructionStatus::Operating,
            influence_radius: 50.,
        })
        .id();
    commands.spawn().insert(Connection {
        between: (
            base_construction_entity.clone(),
            collector2_construction_entity.clone(),
        ),
    });
    commands.spawn().insert(Connection {
        between: (
            collector1_construction_entity.clone(),
            collector2_construction_entity.clone(),
        ),
    });

    let collector3_construction_entity = commands
        .spawn()
        .insert(Construction {
            location: Vec2::new(200., -75.),
            kind: ConstructionKind::Extractor,
            status: ConstructionStatus::Operating,
            influence_radius: 50.,
        })
        .id();
    commands.spawn().insert(Connection {
        between: (
            base_construction_entity.clone(),
            collector3_construction_entity.clone(),
        ),
    });
}

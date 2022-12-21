use bevy::{asset::AssetServerSettings, prelude::*};
use game::GamePlugin;
use model::{
    connection::Connection,
    construction::{Construction, ConstructionKind, ConstructionStatus},
    game::GameEvent,
};

#[test]
fn disallow_crossing_connections() {
    let mut app = App::new();
    app.insert_resource(AssetServerSettings {
        asset_folder: "../../assets".to_string(),
        watch_for_changes: false,
    });
    app.add_plugins(DefaultPlugins);
    app.add_plugin(GamePlugin);

    // Spawn initital constructions
    let base_entity = app
        .world
        .spawn()
        .insert(Construction {
            location: Vec2::new(0., 0.),
            kind: ConstructionKind::Base,
            status: ConstructionStatus::Operating,
        })
        .id();
    let construction_top_right_entity = app
        .world
        .spawn()
        .insert(Construction {
            location: Vec2::new(20., 50.),
            kind: ConstructionKind::Extractor,
            status: ConstructionStatus::Operating,
        })
        .id();
    let construction_bottom_right_entity = app
        .world
        .spawn()
        .insert(Construction {
            location: Vec2::new(20., -50.),
            kind: ConstructionKind::Extractor,
            status: ConstructionStatus::Operating,
        })
        .id();

    // Spawn connections for initial construction
    app.world.spawn().insert(Connection::new_between(
        base_entity,
        construction_top_right_entity,
    ));
    app.world.spawn().insert(Connection::new_between(
        base_entity,
        construction_bottom_right_entity,
    ));
    app.world.spawn().insert(Connection::new_between(
        construction_top_right_entity,
        construction_bottom_right_entity,
    ));

    app.update();

    // Now, lets build a new construction and check the connections that are created
    app.world.send_event(GameEvent::BuildConstruction(
        Vec2::new(21., 0.),
        ConstructionKind::Collector,
    ));
    app.update();

    let mut connection_query = app.world.query::<&Connection>();

    // There should be exactly 5 connections: 3 initially created connections and
    // 2 newly added connection because of the newly build construction. Note that there
    // should not be a 6th connection to the base because this would cross an existing connection.
    assert_eq!(5, connection_query.iter(&app.world).len());
}

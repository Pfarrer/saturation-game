use bevy::{asset::AssetServerSettings, prelude::*};
use game::GamePlugin;
use model::{
    construction::{Construction, ConstructionKind, ConstructionStatus},
    game::GameEvent,
};

pub fn prevent_building_if_influence_areas_overlap() {
    let mut app = App::new();
    app.insert_resource(AssetServerSettings {
        asset_folder: "../../assets".to_string(),
        watch_for_changes: false,
    });
    app.add_plugins(DefaultPlugins);
    app.add_plugin(GamePlugin);

    // Spawn initital construction
    app.world.spawn().insert(Construction {
        location: Vec2::new(0., 0.),
        kind: ConstructionKind::Base,
        status: ConstructionStatus::Operating,
    });
    app.update();

    // Now, lets build a new construction which overlaps
    app.world.send_event(GameEvent::BuildConstruction(
        Vec2::new(1., 0.),
        ConstructionKind::Collector,
    ));
    app.update();

    let mut construction_query = app.world.query::<&Construction>();
    assert_eq!(1, construction_query.iter(&app.world).len());

    // Now, lets build a new construction which does not overlap
    app.world.send_event(GameEvent::BuildConstruction(
        Vec2::new(100., 0.),
        ConstructionKind::Collector,
    ));
    app.update();
    assert_eq!(2, construction_query.iter(&app.world).len());
}

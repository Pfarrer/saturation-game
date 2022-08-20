use bevy::{ecs::event::Events, prelude::*};
use game::GamePlugin;
use model::{
    construction::{Construction, ConstructionKind, ConstructionStatus},
    game::{GameEvent, GameMode},
};

#[test]
fn test1() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(GamePlugin);

    // Spawn base construction
    app.world.spawn().insert(Construction {
        location: Vec2::new(0., 0.),
        kind: ConstructionKind::Base,
        status: ConstructionStatus::Operating,
    });

    // Simulate build button click
    app.world
        .resource_mut::<Events<GameEvent>>()
        .send(GameEvent::SwitchToGameMode(GameMode::Building(
            ConstructionKind::Collector,
        )));

    app.update();

    assert_eq!(5, app.world.entities().len());
}

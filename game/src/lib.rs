use bevy::prelude::*;
use connection::ConnectionShapePlugin;
use construction::ConstructionShapePlugin;
use hud::HudPlugin;
use influence::InfluenceShapePlugin;
use model::{construction::Construction, ModelPlugin, RemovalEvent};
use resources::ResourcesPlugin;

mod game_mode;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ModelPlugin)
            .add_startup_system(init_camera_system)
            .add_plugin(ResourcesPlugin)
            .add_plugin(ConstructionShapePlugin)
            .add_plugin(InfluenceShapePlugin)
            .add_plugin(ConnectionShapePlugin)
            .add_plugin(HudPlugin)
            // .add_plugin(CollisionPlugin)
            .add_system(on_construction_remove_system)
            .add_system(game_mode::switch_game_mode_system);
    }
}

fn init_camera_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn on_construction_remove_system(
    mut commands: Commands,
    mut removal_events: EventReader<RemovalEvent<Construction>>,
) {
    for event in removal_events.iter() {
        debug!("Despawning Construction {:?}", event.entity);
        commands.entity(event.entity).despawn();
    }
}

// fn on_influence_collision_system(
//     // mut collision_events: EventReader<CollisionEvent>,
//     mut query: Query<(&mut Construction, &Collisions)>,
// ) {
//     for (mut construction, collisions) in query.iter_mut() {
//         if construction.status == ConstructionStatus::Hovering && !collisions.0.is_empty() {
//             construction.as_mut().location -= Vec2::new(20., 20.);
//         }
//     }
// }

use std::f64::consts::PI;

use bevy::prelude::*;
use construction::ConstructionShapePlugin;
use model::construction::{Construction, ConstructionKind};

struct BlinkerEntity(Option<Entity>);
struct MovementEntity(Entity);

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ConstructionShapePlugin)
        .add_startup_system(init_system)
        .add_system(demo_movement_system)
        .add_system(demo_blinker_system)
        .run();
}

fn init_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let movement_entity = commands
        .spawn()
        .insert(Construction {
            location: Vec2::new(0., 0.),
            kind: ConstructionKind::Base,
            influence_radius: 20.,
        })
        .id();
    commands.insert_resource(MovementEntity(movement_entity));

    commands.insert_resource(BlinkerEntity(None));
}

fn demo_movement_system(
    time: Res<Time>,
    movement_entity: Res<MovementEntity>,
    mut query: Query<&mut Construction>,
) {
    let x = (time.seconds_since_startup() * PI).cos() * 200.;
    query.get_mut(movement_entity.0).unwrap().location = Vec2::new(x as f32, 0.);
}

fn demo_blinker_system(
    mut commands: Commands,
    time: Res<Time>,
    mut blinker_entity: ResMut<BlinkerEntity>,
) {
    if time.seconds_since_startup() as i32 % 2 == 0 {
        // Do not show the blinker
        if let Some(entity) = blinker_entity.0 {
            commands.entity(entity).despawn();
            blinker_entity.0 = None;
        }
    } else {
        // Show the blinker
        if let None = blinker_entity.0 {
            let entity = commands
                .spawn()
                .insert(Construction {
                    location: Vec2::new(50., 100.),
                    kind: ConstructionKind::Base,
                    influence_radius: 20.,
                })
                .id();
            blinker_entity.0 = Some(entity);
        }
    }
}

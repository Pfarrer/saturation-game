use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes::Circle};
use model::{construction::Construction, RemovalEvent};
use model::collision::Collisions;
use model::construction::{ConstructionKind, ConstructionStatus};

mod build_mode;

const Z_VALUE: f32 = 100.;

pub struct ConstructionShapePlugin;

impl Plugin for ConstructionShapePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ShapePlugin)
            .add_system(spawn_construction_shape_system)
            .add_system(update_construction_shape_system)
            .add_system(remove_construction_shape_system)
            .add_system(build_mode::enter_build_mode_system)
            .add_system(build_mode::exit_build_mode_system)
            .add_system(build_mode::build_mode_on_mouse_move_system)
            .add_system(build_mode::build_mode_on_mouse_click_system);
    }
}

#[derive(Component)]
struct ConstructionShapeRef {
    construction_shape: Entity,
}

#[derive(Component, Debug)]
struct ConstructionShape {
    construction: Entity,
}

fn spawn_construction_shape_system(
    mut commands: Commands,
    query: Query<(Entity, &Construction, Option<&Collisions>), Added<Construction>>,
) {
    for (entity, construction, collisions) in query.iter() {
        let construction_circle = Circle {
            radius: 10.,
            center: Vec2::ZERO,
        };
        let color = construction_color(construction, collisions);

        let bundle = GeometryBuilder::build_as(
            &construction_circle,
            DrawMode::Fill(FillMode::color(color)),
            Transform::from_translation(construction.location.extend(Z_VALUE)),
        );

        let shape_entity = commands
            .spawn_bundle(bundle)
            .insert(ConstructionShape {
                construction: entity,
            })
            .id();
        commands.entity(entity).insert(ConstructionShapeRef {
            construction_shape: shape_entity,
        });

        debug!(
            "ConstructionShape spawned for entity: {:?} -> {:?}",
            entity, construction
        );
    }
}

fn construction_color(construction: &Construction, collisions: Option<&Collisions>) -> Color {
    let has_collisions = collisions.map_or(false, |c| !c.0.is_empty());
    return if has_collisions && construction.status == ConstructionStatus::Hovering {
        let mut color = Color::RED;
        color.set_a(0.7);
        color
    } else {
        match construction.kind {
            ConstructionKind::Base => Color::GREEN,
            ConstructionKind::Collector => Color::YELLOW,
            ConstructionKind::Extractor => Color::BLUE,
        }
    }
}

fn update_construction_shape_system(
    mut commands: Commands,
    construction_query: Query<(&ConstructionShapeRef, &Construction, &Collisions), Changed<Construction>>,
    mut query: Query<&mut Transform>,
) {
    for (shape_ref, construction, collisions) in construction_query.iter() {
        let query_result = query.get_mut(shape_ref.construction_shape);
        if let Ok(mut transform) = query_result {
            transform.translation = construction.location.extend(Z_VALUE);

            let color = construction_color(construction, Some(collisions));
            commands.entity(shape_ref.construction_shape).insert(DrawMode::Fill(FillMode::color(color)));
        }
    }
}

fn remove_construction_shape_system(
    mut commands: Commands,
    mut removal_events: EventReader<RemovalEvent<Construction>>,
    query: Query<(Entity, &ConstructionShape)>,
) {
    for event in removal_events.iter() {
        query
            .iter()
            .filter(|(_, construction_shape)| construction_shape.construction == event.entity)
            .for_each(|(shape_entity, _)| {
                debug!(
                    "Despawning ConstructionShape {:?} of Construction {:?}",
                    shape_entity, event.entity
                );
                commands.entity(shape_entity).despawn();
            });
    }
}

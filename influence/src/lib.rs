use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes::Circle};
use model::{construction::Construction, RemovalEvent};

const Z_VALUE: f32 = 100.;

pub struct InfluenceShapePlugin;

impl Plugin for InfluenceShapePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_influence_shape_system)
            .add_system(update_influence_shape_system)
            .add_system_to_stage(CoreStage::PostUpdate, remove_influence_shape_system);
    }
}

#[derive(Component)]
struct InfluenceShapeRef {
    influence_shape: Entity,
}

#[derive(Component, Debug)]
struct InfluenceShape {
    construction: Entity,
}

fn spawn_influence_shape_system(
    mut commands: Commands,
    query: Query<(Entity, &Construction), Added<Construction>>,
) {
    for (entity, construction) in query.iter() {
        let construction_circle = Circle {
            radius: construction.influence_radius,
            center: Vec2::ZERO,
        };
        let color = Color::Rgba {
            red: 100.,
            green: 200.,
            blue: 50.,
            alpha: 0.1,
        };

        let bundle = GeometryBuilder::build_as(
            &construction_circle,
            DrawMode::Fill(FillMode::color(color)),
            Transform::from_translation(construction.location.extend(Z_VALUE)),
        );

        let shape_entity = commands
            .spawn_bundle(bundle)
            .insert(InfluenceShape {
                construction: entity,
            })
            .id();
        commands.entity(entity).insert(InfluenceShapeRef {
            influence_shape: shape_entity,
        });

        debug!(
            "Entity {:?} added, InfluenceShape spawned for {:?}",
            entity, construction
        );
    }
}

fn update_influence_shape_system(
    construction_query: Query<(&InfluenceShapeRef, &Construction), Changed<Construction>>,
    mut transform_query: Query<&mut Transform>,
) {
    for (shape_ref, construction) in construction_query.iter() {
        transform_query
            .get_mut(shape_ref.influence_shape)
            .unwrap()
            .translation = construction.location.extend(Z_VALUE);
    }
}

fn remove_influence_shape_system(
    mut commands: Commands,
    mut removal_events: EventReader<RemovalEvent<Construction>>,
    query: Query<(Entity, &InfluenceShape)>,
) {
    for event in removal_events.iter() {
        query
            .iter()
            .filter(|(_, influence_shape)| influence_shape.construction == event.entity)
            .for_each(|(shape_entity, _)| {
                debug!(
                    "Despawning InfluenceShape of Construction {:?}",
                    shape_entity
                );
                commands.entity(shape_entity).despawn();
            });
    }
}

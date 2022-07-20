use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes::Circle};
use model::construction::Construction;

const Z_VALUE: f32 = 100.;

pub struct ConstructionShapePlugin;

impl Plugin for ConstructionShapePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ShapePlugin)
            .add_system(spawn_construction_shape_system)
            .add_system(update_construction_shape_system)
            .add_system_to_stage(CoreStage::PostUpdate, remove_construction_shape_system);
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
    query: Query<(Entity, &Construction), Added<Construction>>,
) {
    for (entity, construction) in query.iter() {
        let construction_circle = Circle {
            radius: 10.,
            center: Vec2::ZERO,
        };
        let color = Color::GREEN;

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
            "Entity {:?} added, ConstructionShape spawned for {:?}",
            entity, construction
        );
    }
}

fn update_construction_shape_system(
    construction_query: Query<(&ConstructionShapeRef, &Construction), Changed<Construction>>,
    mut transform_query: Query<&mut Transform>,
) {
    for (shape_ref, construction) in construction_query.iter() {
        transform_query
            .get_mut(shape_ref.construction_shape)
            .unwrap()
            .translation = construction.location.extend(Z_VALUE);
    }
}

fn remove_construction_shape_system(
    mut commands: Commands,
    removals: RemovedComponents<Construction>,
    query: Query<(Entity, &ConstructionShape)>,
) {
    for entity in removals.iter() {
        query
            .iter()
            .filter(|(_, construction_shape)| construction_shape.construction == entity)
            .for_each(|(shape_entity, _)| {
                debug!(
                    "Despawning ConstrctionShape of Construction {:?}",
                    shape_entity
                );
                commands.entity(shape_entity).despawn();
            });
    }
}

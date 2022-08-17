use bevy::{prelude::*, utils::HashSet};
use model::{
    collision::{CollisionAware, CollisionEvent, CollisionStatus, Collisions},
    construction::Construction,
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(on_construction_enter_system)
            .add_system(update_construction_collisions_system);
    }
}

fn on_construction_enter_system(mut commands: Commands, query: Query<Entity, Added<Construction>>) {
    for entity in query.iter() {
        debug!(
            "Collision detection enabled for added Construction entity: {:?}",
            entity
        );
        commands
            .entity(entity)
            .insert(CollisionAware)
            .insert(Collisions(HashSet::new()));
    }
}

fn update_construction_collisions_system(
    changed_query: Query<(Entity, &Construction), (Changed<Construction>, With<CollisionAware>)>,
    construction_query: Query<(Entity, &Construction), With<CollisionAware>>,
    mut collisions_query: Query<&mut Collisions>,
    mut event_writer: EventWriter<CollisionEvent>,
) {
    for (changed_entity, changed_construction) in changed_query.iter() {
        // Drain existing collisions
        let previous_collision: HashSet<_> = {
            let mut collisions = collisions_query.get_mut(changed_entity).unwrap();
            collisions.as_mut().0.drain().collect()
        };

        // (Re-)Add collisions
        let other_constructions = construction_query
            .iter()
            .filter(|(entity, _)| *entity != changed_entity)
            .filter(|(_, construction)| {
                logic::construction::collides_with(construction, changed_construction)
            });
        for (other_entity, _) in other_constructions {
            let mut changed_collisions = collisions_query.get_mut(changed_entity).unwrap();
            changed_collisions.as_mut().0.insert(other_entity);

            let mut other_collisions = collisions_query.get_mut(other_entity).unwrap();
            other_collisions.as_mut().0.insert(changed_entity);
        }

        let updated_collisions = collisions_query.get(changed_entity).unwrap();

        // Send event for new/added collisions
        for other_entity in updated_collisions.0.difference(&previous_collision) {
            event_writer.send(CollisionEvent {
                status: CollisionStatus::Started,
                entities: (changed_entity, *other_entity),
            });
        }

        // Send event for removed collisions
        for other_entity in previous_collision.difference(&updated_collisions.0) {
            event_writer.send(CollisionEvent {
                status: CollisionStatus::Ended,
                entities: (changed_entity, *other_entity),
            });
        }
    }
}

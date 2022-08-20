use bevy::prelude::*;
use model::connection::Connection;
use model::construction::{Construction, ConstructionStatus};
use model::game_configuration::GameConfiguration;
use model::RemovalEvent;

pub fn collides_with(
    game_configuration: &GameConfiguration,
    construction: &Construction,
    other_construction: &Construction,
) -> bool {
    other_construction
        .location
        .distance(construction.location.clone())
        + 0.1 // account for floating point math errors
        < (game_configuration.influence_radius(&other_construction.kind) + game_configuration.influence_radius(&construction.kind))
}

pub(crate) fn on_construction_remove_system(
    mut commands: Commands,
    mut removal_events: EventReader<RemovalEvent<Construction>>,
) {
    for event in removal_events.iter() {
        debug!("Despawning Construction {:?}", event.entity);
        commands.entity(event.entity).despawn();
    }
}

pub(crate) fn build_construction(
    commands: &mut Commands,
    game_configuration: &Res<GameConfiguration>,
    construction_query: &Query<(Entity, &Construction)>,
    location: &Vec2,
    kind: &model::construction::ConstructionKind,
) {
    let new_construction_entity = commands
        .spawn()
        .insert(Construction {
            location: location.clone(),
            kind: kind.clone(),
            status: ConstructionStatus::Operating,
        })
        .id();

    construction_query
        .iter()
        .filter(|(_, construction)| {
            let distance = construction.location.distance(*location);
            distance > 0. && distance < game_configuration.max_connection_distance()
        })
        .for_each(|(entity_in_range, _)| {
            commands.spawn().insert(Connection {
                between: (entity_in_range, new_construction_entity),
            });
        });
}

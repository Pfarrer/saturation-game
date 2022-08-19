use bevy::prelude::*;
use model::construction::Construction;
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

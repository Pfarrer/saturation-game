use bevy::prelude::*;
use model::{RemovalEvent, connection::Connection};

pub(crate) fn on_connection_remove_system(
    mut commands: Commands,
    mut removal_events: EventReader<RemovalEvent<Connection>>,
) {
    for event in removal_events.iter() {
        debug!("Despawning Connection {:?}", event.entity);
        commands.entity(event.entity).despawn();
    }
}
use bevy::prelude::*;
use model::{connection::Connection, construction::Construction, RemovalEvent};

/// On construction remove, query all connections and remove all that were connected
/// to the construction to be removed.
pub(crate) fn on_construction_remove_system(
    mut construction_removal_events: EventReader<RemovalEvent<Construction>>,
    mut connection_event_writer: EventWriter<RemovalEvent<Connection>>,
    connection_query: Query<(Entity, &Connection)>,
) {
    for event in construction_removal_events.iter() {
        debug!("construction removal event: {:?}", event);
        connection_query
            .iter()
            .filter(|(_, connection)| connection.connects_to(event.entity))
            .for_each(|(connection_entity, connection)| {
                debug!(
                    "Removing {:?} because {:?} removed",
                    connection, event.component
                );
                connection_event_writer.send(RemovalEvent {
                    entity: connection_entity,
                    component: connection.clone(),
                });
            });
    }
}

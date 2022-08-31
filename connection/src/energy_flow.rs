use bevy::prelude::*;
use model::{construction::Construction, connection::Connection, RemovalEvent, game_configuration::GameConfiguration};

#[derive(Default)]
pub struct UpdateConnectionEnergyFlows;

pub fn emit_update_connection_energy_flows_system(
    construction_trigger_query: Query<(), Or<(Added<Construction>, Changed<Construction>)>>,
    connection_trigger_query: Query<(), Added<Connection>>,
    construction_removal_events: EventReader<RemovalEvent<Construction>>,
    connection_removal_events: EventReader<RemovalEvent<Connection>>,
    mut event_writer: EventWriter<UpdateConnectionEnergyFlows>,
) {
    if construction_trigger_query.is_empty()
        && connection_trigger_query.is_empty()
        && construction_removal_events.is_empty()
        && connection_removal_events.is_empty()
    {
        return;
    }

    construction_removal_events.clear();
    connection_removal_events.clear();

    event_writer.send_default();
}

pub fn on_update_connection_energy_flows_system(
    event_reader: EventReader<UpdateConnectionEnergyFlows>,
    construction_query: Query<(Entity, &Construction)>,
    mut connection_query: Query<&mut Connection>,
    game_configuration: Res<GameConfiguration>,
) {
    if event_reader.is_empty() {
        return;
    }
    event_reader.clear();

    trace!("Updating energy flow values in all connections...");

    let constructions = construction_query.iter().collect();
    let connections = connection_query.iter().collect();

    // Assign energy flows to each connection
    let energy_flows = logic::connection_energy_flow::assign_energy_flows(
        &game_configuration,
        &constructions,
        &connections,
    );

    // Map assigned energy flows to the connection objects
    for mut connection in connection_query.iter_mut() {
        connection.as_mut().energy_flow = energy_flows
            .get(connection.between())
            .copied()
            .unwrap_or(0.);
    }
}

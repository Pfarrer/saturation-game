use bevy::prelude::*;
use geo::line_intersection::line_intersection;
use geo::{coord, Line};
use model::connection::Connection;
use model::construction::{Construction, ConstructionKind, ConstructionStatus};
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
    connection_query: &Query<&Connection>,
    location: &Vec2,
    kind: &ConstructionKind,
) {
    let new_construction_entity = commands
        .spawn()
        .insert(Construction {
            location: location.clone(),
            kind: kind.clone(),
            status: ConstructionStatus::Operating,
        })
        .id();

    let existing_connection_lines: Vec<_> = connection_query
        .iter()
        .map(|connection| {
            let (_, construction1) = construction_query.get(connection.between().0).unwrap();
            let (_, construction2) = construction_query.get(connection.between().1).unwrap();
            Line::new(
                coord! { x: construction1.location.x, y: construction1.location.y },
                coord! { x: construction2.location.x, y: construction2.location.y },
            )
        })
        .collect();

    construction_query
        .iter()
        .filter(|(_, construction)| {
            let distance = construction.location.distance(*location);
            distance > 0. && distance < game_configuration.max_connection_distance()
        })
        .map(|(entity_in_range, construction)| {
            let connection = Connection::new_between(entity_in_range, new_construction_entity);
            (connection, construction)
        })
        .filter(|(_, construction)| {
            existing_connection_lines
                .iter()
                .all(|existing_connection_line| {
                    let connection_line_candidate = Line::new(
                        coord! { x: construction.location.x, y: construction.location.y },
                        coord! { x: location.x, y: location.y },
                    );
                    let intersection =
                        line_intersection(*existing_connection_line, connection_line_candidate);
                    !intersection
                        .map(|intersect| intersect.is_proper())
                        .unwrap_or(false)
                })
        })
        .for_each(|(connection, _)| {
            commands.spawn().insert(connection);
        });
}

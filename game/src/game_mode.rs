use bevy::prelude::*;
use model::{
    connection::Connection,
    construction::{Construction, ConstructionStatus},
    game::{GameEvent, GameMode},
};

pub(crate) fn switch_game_mode_system(
    mut commands: Commands,
    mut game_events: EventReader<GameEvent>,
    current_game_mode: Res<GameMode>,
    construction_query: Query<(Entity, &Construction)>,
) {
    for event in game_events.iter() {
        debug!("Processing game event: {:?}", event);
        match event {
            &GameEvent::SwitchToGameMode(ref target_mode) => {
                match (current_game_mode.as_ref(), target_mode) {
                    (&GameMode::Idle, _) => commands.insert_resource(target_mode.clone()),
                    (&GameMode::Building(_), &GameMode::Idle) => commands.insert_resource(target_mode.clone()),
                    _ => todo!("Switch from {:?} to {:?}", current_game_mode.as_ref(), target_mode),
                };
            }
            &GameEvent::BuildConstruction(ref location, ref kind) => {
                build_construction(&mut commands, &construction_query, location, kind);
                commands.insert_resource(GameMode::Idle);
            } // _ => panic!("Game event type not implemented: {:?}", event),
        }
    }
}

fn build_construction(
    commands: &mut Commands,
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
            influence_radius: 20.,
        })
        .id();

    let mut other_construction_entities = construction_query
        .iter()
        .filter(|(_, construction)| construction.location != *location)
        .map(|(entity, construction)| (entity, construction.location.distance(*location)))
        .collect::<Vec<_>>();
    other_construction_entities.sort_by(|(_, distance1), (_, distance2)| {
        distance1
            .partial_cmp(distance2)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let closest_construction_entity = other_construction_entities.first().unwrap().0;

    commands.spawn().insert(Connection {
        between: (closest_construction_entity, new_construction_entity),
    });
}

use bevy::prelude::*;
use model::game_configuration::GameConfiguration;
use model::{
    construction::Construction,
    game::{GameEvent, GameMode},
};

pub(crate) fn switch_game_mode_system(
    mut commands: Commands,
    mut game_events: EventReader<GameEvent>,
    current_game_mode: Res<GameMode>,
    game_configuration: Res<GameConfiguration>,
    construction_query: Query<(Entity, &Construction)>,
) {
    for event in game_events.iter() {
        debug!("Processing game event: {:?}", event);
        match event {
            &GameEvent::SwitchToGameMode(ref target_mode) => {
                match (current_game_mode.as_ref(), target_mode) {
                    (&GameMode::Idle, _) => commands.insert_resource(target_mode.clone()),
                    (&GameMode::Building(_), &GameMode::Idle) => {
                        commands.insert_resource(target_mode.clone())
                    }
                    _ => todo!(
                        "Switch from {:?} to {:?}",
                        current_game_mode.as_ref(),
                        target_mode
                    ),
                };
            }
            &GameEvent::BuildConstruction(ref location, ref kind) => {
                crate::construction::build_construction(
                    &mut commands,
                    &game_configuration,
                    &construction_query,
                    location,
                    kind,
                );
                commands.insert_resource(GameMode::Idle);
            } // _ => panic!("Game event type not implemented: {:?}", event),
        }
    }
}

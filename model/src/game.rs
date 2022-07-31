use bevy::prelude::*;

use crate::construction::ConstructionKind;

#[derive(Component, Debug, Clone, PartialEq)]
pub enum GameMode {
    Idle,
    Building(ConstructionKind),
}

pub(crate) fn game_mode_debug_system(game_mode: Res<GameMode>) {
    if game_mode.is_changed() {
        debug!("Game Mode changed: {:?}", game_mode);
    }
}

#[derive(Component, Debug, Clone, PartialEq)]
pub enum GameEvent {
    SwitchToGameMode(GameMode),
    BuildConstruction(Vec2, ConstructionKind),
}

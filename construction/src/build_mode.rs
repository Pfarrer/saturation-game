use model::{
    construction::{Construction, ConstructionStatus, UnderConstructionMarker},
    game::{GameEvent, GameMode},
    RemovalEvent,
};

use bevy::{input::mouse::MouseMotion, prelude::*, render::camera::Camera2d};

pub(crate) fn enter_build_mode_system(
    mut commands: Commands,
    game_mode: Res<GameMode>,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    if !game_mode.is_changed() {
        return;
    }

    if let GameMode::Building(kind) = game_mode.as_ref() {
        commands
            .spawn()
            .insert(Construction {
                location: logic::window_to_world(&windows, &camera_query).unwrap(),
                kind: kind.clone(),
                status: ConstructionStatus::Hovering,
                influence_radius: 20.,
            })
            .insert(UnderConstructionMarker);
    }
}

pub(crate) fn exit_build_mode_system(
    game_mode: Res<GameMode>,
    query: Query<(Entity, &Construction), With<UnderConstructionMarker>>,
    mut event_writer: EventWriter<RemovalEvent<Construction>>,
) {
    if !game_mode.is_changed() {
        return;
    }

    query.iter().for_each(|(entity, construction)| {
        event_writer.send(RemovalEvent {
            entity: entity,
            component: construction.clone(),
        })
    });
}

pub(crate) fn build_mode_on_mouse_move_system(
    mut under_construction_query: Query<&mut Construction, With<UnderConstructionMarker>>,
    other_constructions_query: Query<&Construction, Without<UnderConstructionMarker>>,
    mouse_motion_events: EventReader<MouseMotion>,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    if mouse_motion_events.is_empty() {
        return;
    }

    for mut construction in under_construction_query.iter_mut() {
        if let Some(world_location) = logic::window_to_world(&windows, &camera_query) {
            logic::construction::move_to(
                &world_location,
                construction.as_mut(),
                other_constructions_query.iter().collect(),
            );
        }
    }
}

pub(crate) fn build_mode_on_mouse_click_system(
    under_construction_query: Query<&Construction, With<UnderConstructionMarker>>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut game_event_writer: EventWriter<GameEvent>,
) {
    for construction in under_construction_query.iter() {
        if mouse_buttons.just_pressed(MouseButton::Left) {
            game_event_writer.send(GameEvent::BuildConstruction(
                construction.location,
                construction.kind.clone(),
            ));
        }
    }
}

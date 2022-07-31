use model::{
    construction::ConstructionKind,
    game::{GameEvent, GameMode},
};

use bevy::{prelude::*, render::camera::Camera2d};
use bevy_prototype_lyon::{prelude::*, shapes::Circle};

use crate::Z_VALUE;

#[derive(Component)]
pub(crate) struct BuildModeMarker(ConstructionKind);

pub(crate) fn enter_exit_build_mode_system(
    mut commands: Commands,
    game_mode: Res<GameMode>,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    query: Query<Entity, With<BuildModeMarker>>,
) {
    if !game_mode.is_changed() {
        return;
    }

    if let GameMode::Building(kind) = game_mode.as_ref() {
        let construction_circle = Circle {
            radius: 10.,
            center: Vec2::ZERO,
        };
        let color = Color::GREEN;
        let cursor_position = window_to_world(&windows, &camera_query).unwrap();

        let bundle = GeometryBuilder::build_as(
            &construction_circle,
            DrawMode::Fill(FillMode::color(color)),
            Transform::from_translation(cursor_position.extend(Z_VALUE)),
        );
        commands
            .spawn_bundle(bundle)
            .insert(BuildModeMarker(kind.clone()));
    } else {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

pub(crate) fn build_mode_update_system(
    mut query: Query<(&mut Transform, &BuildModeMarker)>,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut game_event_writer: EventWriter<GameEvent>,
) {
    for (mut transform, build_mode_marker) in query.iter_mut() {
        if let Some(new_location) = window_to_world(&windows, &camera_query) {
            transform.as_mut().translation = new_location.extend(Z_VALUE);
        }

        if mouse_buttons.just_pressed(MouseButton::Left) {
            game_event_writer.send(GameEvent::BuildConstruction(
                transform.translation.truncate(),
                build_mode_marker.0.clone(),
            ));
        }
    }
}

fn window_to_world(
    wnds: &Res<Windows>,
    camera_query: &Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) -> Option<Vec2> {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_query.single();
    let wnd = wnds.get_primary().unwrap();

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();
        Some(world_pos)
    } else {
        None
    }
}

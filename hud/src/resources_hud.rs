use bevy::prelude::*;
use model::resources::PlayerResources;

#[derive(Component)]
pub(crate) struct ResourcesHud;

pub(crate) fn create_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(10.),
                    right: Val::Px(10.),
                    ..default()
                },
                ..default()
            },
            text: Text::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/OpenSans/OpenSans.ttf"),
                    font_size: 25.,
                    color: Color::YELLOW,
                }
            ),
            ..default()
        })
        .insert(ResourcesHud);
}

pub(crate) fn update_hud(
    mut hud_query: Query<&mut Text, With<ResourcesHud>>,
    resources: Res<PlayerResources>,
) {
    for mut hud in hud_query.iter_mut() {
        hud.sections[0].value = format!(
            "{:.0} Material (+{:.1}/s), Energy: {}/{} ({:.1}%)",
            resources.material_available,
            resources.material_rate_per_second,
            resources.energy_need,
            resources.energy_available,
            resources.energy_need / resources.energy_available * 100.
        );
    }
}

use bevy::prelude::*;
use model::resources::PlayerResources;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_hud).add_system(update_hud);
    }
}

fn create_hud(mut commands: Commands, font_assets: Res<AssetServer>) {
    commands.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        },
        text: Text::with_section(
            "Resources: ",
            TextStyle {
                font: font_assets.load("fonts/OpenSans.ttf"),
                font_size: 25.,
                color: Color::YELLOW,
            },
            default(),
        ),
        ..default()
    });
}

fn update_hud(mut hud_query: Query<&mut Text>, resources: Res<PlayerResources>) {
    for mut hud in hud_query.iter_mut() {
        hud.sections[0].value = format!("Resources: {:.0} Material", resources.material);
    }
}

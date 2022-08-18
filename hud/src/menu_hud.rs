use bevy::prelude::*;
use model::{
    construction::ConstructionKind,
    game::{GameEvent, GameMode},
};

#[derive(Component, Debug)]
pub(crate) struct BuildButton(ConstructionKind);

pub(crate) fn create_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(elements::root_node_bundle())
        .with_children(|parent| {
            parent.spawn_bundle(elements::button_text_bundle("Build:", &asset_server));

            parent
                .spawn_bundle(elements::button_bundle())
                .insert(BuildButton(ConstructionKind::Extractor))
                .with_children(|parent| {
                    parent.spawn_bundle(elements::button_text_bundle("Extractor", &asset_server));
                });

            parent
                .spawn_bundle(elements::button_bundle())
                .insert(BuildButton(ConstructionKind::Collector))
                .with_children(|parent| {
                    parent.spawn_bundle(elements::button_text_bundle("Collector", &asset_server));
                });
        });
}

pub(crate) fn menu_hud_system(
    mut interaction_query: Query<(&Interaction, &BuildButton), Changed<Interaction>>,
    mut game_event_writer: EventWriter<GameEvent>,
) {
    for (interaction, build_button) in interaction_query.iter_mut() {
        if interaction == &Interaction::Clicked {
            let target_mode = GameMode::Building(build_button.0.clone());
            game_event_writer.send(GameEvent::SwitchToGameMode(target_mode));
        }
    }
}

mod elements {
    use bevy::prelude::*;

    pub(crate) fn root_node_bundle() -> NodeBundle {
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(40.), Val::Px(40.)),
                padding: UiRect::all(Val::Px(5.)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Stretch,
                ..default()
            },
            color: UiColor(Color::rgba_u8(223, 223, 223, (0.4 * u8::MAX as f32) as u8)),
            ..default()
        }
    }

    pub(crate) fn button_bundle() -> ButtonBundle {
        ButtonBundle {
            style: Style {
                // size: Size::new(Val::Px(200.), Val::Px(30.)),
                // margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                // align_items: AlignItems::Center,
                align_items: AlignItems::Stretch,
                align_self: AlignSelf::Center,
                padding: UiRect::all(Val::Px(2.)),
                // position_type: PositionType::Absolute,
                // position: UiRect {
                //     bottom: Val::Px(10.),
                //     left: Val::Px(10.),
                //     ..default()
                // },
                ..default()
            },
            ..default()
        }
    }

    pub(crate) fn button_text_bundle(text: &str, asset_server: &Res<AssetServer>) -> TextBundle {
        TextBundle {
            text: Text::from_section(
                text,
                TextStyle {
                    font: asset_server.load("fonts/OpenSans/OpenSans.ttf"),
                    font_size: 25.,
                    color: Color::BLACK,
                }
            ),
            ..default()
        }
    }
}

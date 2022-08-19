use bevy::prelude::*;
use model::{
    construction::{Construction, ConstructionStatus},
    game::GameEvent,
    game_configuration::GameConfiguration,
    resources::PlayerResources,
};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_system)
            .add_system(update_available_resources)
            .add_system(on_game_event_resource_system);
    }
}

fn init_system(mut commands: Commands, init_player_resources: Option<Res<PlayerResources>>) {
    if let None = init_player_resources {
        commands.insert_resource(PlayerResources {
            material_available: 100.,
            material_rate_per_second: 0.,
            energy_need: 0.,
            energy_available: 0.,
        });
    }
}

fn update_available_resources(
    time: Res<Time>,
    query: Query<&Construction>,
    mut player_resources: ResMut<PlayerResources>,
    game_configuration: Res<GameConfiguration>,
) {
    let time_delta = time.delta().as_secs_f64();

    // material_rate_per_second & material_available
    let material_rate_per_second = query
        .iter()
        .filter(|construction| construction.status == ConstructionStatus::Operating)
        .fold(0., |acc, construction| {
            acc + game_configuration.material_output(&construction.kind)
        });
    player_resources.as_mut().material_rate_per_second = material_rate_per_second;
    player_resources.as_mut().material_available += time_delta * material_rate_per_second;

    // energy_need
    let energy_need = query
        .iter()
        .filter(|construction| construction.status == ConstructionStatus::Operating)
        .fold(0., |acc, construction| {
            acc + game_configuration.energy_input(&construction.kind)
        });
    player_resources.as_mut().energy_need = energy_need;

    // energy_available
    let energy_available = query
        .iter()
        .filter(|construction| construction.status == ConstructionStatus::Operating)
        .fold(0., |acc, construction| {
            acc + game_configuration.energy_output(&construction.kind)
        });
    player_resources.as_mut().energy_available = energy_available;
}

fn on_game_event_resource_system(
    mut game_events: EventReader<GameEvent>,
    mut player_resources: ResMut<PlayerResources>,
    game_configuration: Res<GameConfiguration>,
) {
    for event in game_events.iter() {
        match event {
            &GameEvent::BuildConstruction(_, ref kind) => {
                player_resources.as_mut().material_available -=
                    game_configuration.material_build_demand(kind);
            }
            _ => (),
        }
    }
}

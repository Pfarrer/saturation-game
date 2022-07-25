use bevy::prelude::*;
use model::{
    construction::{Construction, ConstructionKind},
    resources::PlayerResources,
};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_system)
            .add_system(update_available_resources);
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
) {
    let time_delta = time.delta().as_secs_f64();

    // material_available
    let material_rate_per_second = query
        .iter()
        .filter(|c| c.kind == ConstructionKind::Extractor)
        .fold(0., |acc, _| acc + 1.);
    player_resources.as_mut().material_rate_per_second = material_rate_per_second;
    player_resources.as_mut().material_available += time_delta * material_rate_per_second;

    // energy_need
    let (energy_need, energy_available) =
        query
            .iter()
            .fold((0., 0.), |acc, construction| match construction.kind {
                ConstructionKind::Base => (acc.0, acc.1 + 10.),
                ConstructionKind::Collector => (acc.0, acc.1 + 5.),
                ConstructionKind::Extractor => (acc.0 + 3., acc.1),
            });
    player_resources.as_mut().energy_need = energy_need;
    player_resources.as_mut().energy_available = energy_available;
}

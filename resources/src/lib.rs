use bevy::prelude::*;
use model::{construction::Construction, resources::PlayerResources};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_system)
            .add_system(update_resources);
    }
}

fn init_system(mut commands: Commands, init_player_resources: Option<Res<PlayerResources>>) {
    if let None = init_player_resources {
        commands.insert_resource(PlayerResources { material: 100. });
    }
}

fn update_resources(
    time: Res<Time>,
    mut player_resources: ResMut<PlayerResources>,
    query: Query<&Construction>,
) {
    let time_delta = time.delta().as_secs_f64();
    for _construction in query.iter() {
        player_resources.as_mut().material += time_delta * 1.;
    }
}

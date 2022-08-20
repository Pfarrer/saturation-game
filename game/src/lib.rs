use bevy::prelude::*;
use collision::CollisionPlugin;
use connection::ConnectionShapePlugin;
use construction::ConstructionShapePlugin;
use hud::HudPlugin;
use influence::InfluenceShapePlugin;
use logic::LogicPlugin;
use model::ModelPlugin;
use resources::ResourcesPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ModelPlugin)
            .add_startup_system(init_camera_system)
            .add_plugin(ResourcesPlugin)
            .add_plugin(ConstructionShapePlugin)
            .add_plugin(InfluenceShapePlugin)
            .add_plugin(ConnectionShapePlugin)
            .add_plugin(HudPlugin)
            .add_plugin(CollisionPlugin)
            .add_plugin(LogicPlugin);
    }
}

fn init_camera_system(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

use bevy::prelude::*;

mod menu_hud;
mod resources_hud;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(resources_hud::create_hud)
            .add_system(resources_hud::update_hud)
            .add_startup_system(menu_hud::create_hud)
            .add_system(menu_hud::menu_hud_system);
    }
}

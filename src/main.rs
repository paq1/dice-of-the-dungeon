use bevy::prelude::*;

mod state;
mod plugins;

use crate::state::AppState;
use crate::plugins::{main_menu_plugin::MainMenuPlugin, in_game_plugin::InGamePlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(
            WindowDescriptor {
                title: "dice of the dungeon -- gamejam".to_string(),
                width: 600.,
                height: 600.,
                ..Default::default()
            }
        )
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup_system)
        .add_state(AppState::MainMenu)
        .add_plugin(MainMenuPlugin)
        .add_plugin(InGamePlugin)
        .run();
}

fn startup_system(
    mut commands: Commands
) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

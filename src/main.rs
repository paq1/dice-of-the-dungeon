use bevy::prelude::*;
use crate::models::stat_player::StatsPlayer;
use crate::models::win_size::WinSize;

mod state;
mod plugins;
mod models;

mod tests;

use crate::state::AppState;
use crate::plugins::{main_menu_plugin::MainMenuPlugin, in_game_plugin::InGamePlugin};
use crate::plugins::player_plugin::PlayerPlugin;

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
        .add_state(AppState::InGame) // todo set MainMenu
        .add_plugin(MainMenuPlugin)
        .add_plugin(InGamePlugin)
        .add_plugin(PlayerPlugin)
        .run();
}

fn startup_system(
    mut commands: Commands,
    windows: Res<Windows>
) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let window = windows.get_primary().unwrap();
    let (win_w, win_h) = (window.width(), window.height());
    commands.insert_resource(WinSize {width: win_w, height: win_h});

    commands
        .insert_resource(StatsPlayer::default());
}

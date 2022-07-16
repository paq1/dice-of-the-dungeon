use bevy::prelude::*;
use crate::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::MainMenu)
                    .with_system(print_on_menu_system)
            )
            .add_system_set(
                SystemSet::on_update(AppState::MainMenu)
                    .with_system(switch_to_in_game_state_system)
            );
    }
}

fn print_on_menu_system() {
    println!("-- on main menu --");
}

fn switch_to_in_game_state_system(
    mut app_state: ResMut<State<AppState>>,
    keyboard: Res<Input<KeyCode>>
) {
    if keyboard.just_pressed(KeyCode::Space) {
        println!("changement d'état : main menu --> in game");
        app_state.set(AppState::InGame).unwrap();
    }
}
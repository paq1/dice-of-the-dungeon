use bevy::prelude::*;
use crate::AppState;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::InGame)
                    .with_system(print_on_in_game_system)
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(switch_to_main_menu_state_system)
            );
    }
}

fn print_on_in_game_system() {
    println!("-- on in game --");
}

fn switch_to_main_menu_state_system(
    mut app_state: ResMut<State<AppState>>,
    keyboard: Res<Input<KeyCode>>
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        println!("changement d'état : in game --> main menu");
        app_state.set(AppState::MainMenu).unwrap();
    }
}
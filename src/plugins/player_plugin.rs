use bevy::prelude::*;
use crate::AppState;
use crate::models::stat_player::StatsPlayer;

pub struct PlayerPlugin;

#[derive(Component)]
struct TextStatPlayer;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::InGame)
                    .with_system(insert_player_stat_system)
                    .with_system(init_text_stat_player_system)
            );
    }
}

fn insert_player_stat_system(
    mut commands: Commands
) {
    // le joueur est une ressource car il n'a pas sprite associé
    commands
        .insert_resource(StatsPlayer::default());
    println!(" -- player resource loaded");

}

fn init_text_stat_player_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let pos_stat_player = (0.0, 0.0);

    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "stats : ".to_string(),
                        style: TextStyle {
                            color: Color::ORANGE,
                            font: asset_server.load("PaintDrops.ttf"),
                            font_size: 36.
                        }
                    }
                ],
                ..Default::default()
            },
            transform: Transform::from_xyz(pos_stat_player.0, pos_stat_player.1, 2.),
            ..Default::default()
        })
        .insert(TextStatPlayer);
}
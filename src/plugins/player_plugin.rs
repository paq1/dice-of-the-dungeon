use bevy::prelude::*;
use crate::{AppState, WinSize};
use crate::models::stat_player::StatsPlayer;

pub struct PlayerPlugin;

#[derive(Component)]
struct TextStatsPlayer;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::InGame)
                    .with_system(init_text_stat_player_system)
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(update_text_stat_system)
            );
    }
}

fn init_text_stat_player_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    win_size: Res<WinSize>,
    stats: Res<StatsPlayer>
) {
    let pos_stat_player = (64., -win_size.height / 2. + 18.);

    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: stat_text(stats.hp, stats.shield),
                        style: TextStyle {
                            color: Color::GREEN,
                            font: asset_server.load("PaintDrops.ttf"),
                            font_size: 18.
                        }
                    }
                ],
                ..Default::default()
            },
            transform: Transform::from_xyz(pos_stat_player.0, pos_stat_player.1, 2.),
            ..Default::default()
        })
        .insert(TextStatsPlayer);
}

fn update_text_stat_system(
    stats: Res<StatsPlayer>,
    mut stats_text_query: Query<(Entity, &mut Text), With<TextStatsPlayer>>
) {
    stats_text_query.iter_mut()
        .for_each(|(_, mut text)| {
            text.sections[0].value = stat_text(stats.hp, stats.shield);
        });
}

fn stat_text(hp: u32, shield: u32) -> String {
    format!("hp : ({hp}) -- shield ({shield})")
}
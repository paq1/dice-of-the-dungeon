use crate::models::stat_player::StatsPlayer;

#[test]
fn should_have_100_hp_and_25_shield_when_player_default() {
    // given

    // when
    let player = StatsPlayer::default();

    // then
    assert_eq!(player.hp, 100);
    assert_eq!(player.shield, 25);
}
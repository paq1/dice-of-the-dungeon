const HP_PLAYER_AT_BEGIN: u32 = 100;
const SHIELD_PLAYER_AT_BEGIN: u32 = 25;

pub struct StatsPlayer {
    pub hp: u32,
    pub shield: u32
}

impl Default for StatsPlayer {
    fn default() -> Self {
        StatsPlayer {
            hp: HP_PLAYER_AT_BEGIN,
            shield: SHIELD_PLAYER_AT_BEGIN
        }
    }
}
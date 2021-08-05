use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GameConfig {
    pub gravity: f32,
    pub player_speed: f32,
}
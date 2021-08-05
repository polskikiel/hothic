use std::fs;
use serde::Deserialize;
use crate::common::types::GameConfig;

pub fn load_config(file_path: String) -> GameConfig {
    let file = fs::File::open(file_path)
        .expect("file should open read only");
    let result: GameConfig = serde_json::from_reader(file)
        .expect("file should be proper JSON");
    return result;
}
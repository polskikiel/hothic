mod game;
mod player;
mod player_camera;

use gdnative::prelude::{godot_init, InitHandle};

lazy_static! {
    static ref GAMEDATA: gamedata::data::GameDataS =
        gamedata::data::load_data("./src/assets/data.json".to_string());
}

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<game::Game>();
    handle.add_class::<player::Player>();
    handle.add_class::<player_camera::PlayerCamera>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);

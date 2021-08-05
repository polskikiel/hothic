mod game;
mod player;
mod player_camera;
mod common;

use gdnative::prelude::{godot_init, InitHandle};

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<game::Game>();
    handle.add_class::<player::Player>();
    handle.add_class::<player_camera::PlayerCamera>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);

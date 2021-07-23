mod game;
// mod camera;

use gdnative::prelude::{godot_init, InitHandle};

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<game::Game>();
    // handle.add_class::<camera::Camera>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);

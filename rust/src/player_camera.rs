use gdnative::api::*;
use gdnative::prelude::*;
use crate::player::*;

#[derive(NativeClass)]
#[inherit(Camera)]
#[register_with(Self::register_builder)]
pub struct PlayerCamera {
    name: String,
}

// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[methods]
impl PlayerCamera {

    // Register the builder for methods, properties and/or signals.
    fn register_builder(_builder: &ClassBuilder<Self>) {
        godot_print!("PlayerCamera builder is registered!");
    }

    /// The "constructor" of the class.
    fn new(_owner: &Camera) -> Self {
        godot_print!("PlayerCamera is created!");
        PlayerCamera {
            name: "".to_string(),
        }
    }

    // In order to make a method known to Godot, the #[export] attribute has to be used.
    // In Godot script-classes do not actually inherit the parent class.
    // Instead they are "attached" to the parent object, called the "owner".
    // The owner is passed to every single exposed method.
    #[export]
    unsafe fn _ready(&mut self, _owner: &Camera) {
        self.name = "PlayerCamera".to_string();

        // let player_char: TRef<Camera> = unsafe {
        //     _owner.get_node_as::<Camera>("PlayerCameraChar").unwrap()
        // };
        // player_wrapper.

        // self.player_init_vector = _owner.translation();
        // self.player_init_zoom = player_wrapper.transform().length();
        let player_transform = _owner.transform();
        let player_position = player_transform.origin;

        godot_print!("{} is ready!", self.name);
    }

    // This function will be called in every frame
    #[export]
    unsafe fn _process(&self, _owner: &Camera, delta: f64) {
        let input = Input::godot_singleton();

        let player_transform = _owner.transform();
        let mut player_position = player_transform.origin;

        if Input::is_action_pressed(&input, "ui_right") {
            player_position.x -= common::PLAYER_SPEED
        }
        if Input::is_action_pressed(&input, "ui_left") {
            player_position.x += 1.0
        }
        if Input::is_action_pressed(&input, "ui_down") {
            player_position.z -= 1.0
        }
        if Input::is_action_pressed(&input, "ui_up") {
            player_position.z += 1.0
        }
        _owner.set_translation(player_position);
    }
}

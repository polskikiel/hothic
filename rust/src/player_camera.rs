use gdnative::api::*;
use gdnative::prelude::*;
use crate::common::types::*;
use crate::common::config::*;
use crate::common::vars::*;

#[derive(NativeClass)]
#[inherit(Camera)]
#[register_with(Self::register_builder)]
pub struct PlayerCamera {
    name: String,
    gravity: f32,
    player_speed: f32,
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
            name: "PlayerCamera".to_string(),
            gravity: 0.0,
            player_speed: 0.0,
        }
    }

    // In order to make a method known to Godot, the #[export] attribute has to be used.
    // In Godot script-classes do not actually inherit the parent class.
    // Instead they are "attached" to the parent object, called the "owner".
    // The owner is passed to every single exposed method.
    #[export]
    unsafe fn _ready(&mut self, _owner: &Camera) {
        let cfg: GameConfig = load_config(CONFIG_PATH.to_string());

        self.gravity = cfg.gravity;
        self.player_speed = cfg.player_speed;

        godot_print!("{} is ready!", self.name);
        godot_print!("{} speed", self.player_speed);
    }

    // This function will be called in every frame
    #[export]
    unsafe fn _process(&self, _owner: &Camera, delta: f64) {
        let input = Input::godot_singleton();

        let player_transform = _owner.transform();
        let mut player_position = player_transform.origin;

        if Input::is_action_pressed(&input, "ui_right") {
            player_position.x -= self.player_speed
        }
        if Input::is_action_pressed(&input, "ui_left") {
            player_position.x += self.player_speed
        }
        if Input::is_action_pressed(&input, "ui_down") {
            player_position.z -= self.player_speed
        }
        if Input::is_action_pressed(&input, "ui_up") {
            player_position.z += self.player_speed
        }
        if Input::is_action_pressed(&input, "jump") {
            player_position.y += self.player_speed;
        }
        if Input::is_action_pressed(&input, "crouch") {
            player_position.y -= self.player_speed;
        }
        _owner.set_translation(player_position);
    }
}

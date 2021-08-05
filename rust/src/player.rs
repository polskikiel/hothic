use gdnative::api::*;
use gdnative::prelude::*;
use crate::common::types::*;
use crate::common::config::*;
use crate::common::vars::*;
use std::ops::Deref;
use std::borrow::{Borrow, BorrowMut};

#[derive(NativeClass)]
#[inherit(Spatial)]
#[register_with(Self::register_builder)]
pub struct Player {
    name: String,

    gravity: f32,
    player_speed: f32,

    skeleton_ref: Ref<Node>,
    animation_player_ref: Ref<Node>,
    animation_tree_ref: Ref<Node>,

}

// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[methods]
impl Player {
    // Register the builder for methods, properties and/or signals.
    fn register_builder(_builder: &ClassBuilder<Self>) {
        godot_print!("Player builder is registered!");
    }

    /// The "constructor" of the class.
    fn new(_owner: &Spatial) -> Self {
        godot_print!("Player is created!");
        Player {
            name: "Player".to_string(),
            gravity: 0.0,
            player_speed: 0.0,
            skeleton_ref: Node::new().into_shared(),
            animation_player_ref: Node::new().into_shared(),
            animation_tree_ref: Node::new().into_shared(),
        }
    }

    // In order to make a method known to Godot, the #[export] attribute has to be used.
    // In Godot script-classes do not actually inherit the parent class.
    // Instead they are "attached" to the parent object, called the "owner".
    // The owner is passed to every single exposed method.
    #[export]
    unsafe fn _ready(&mut self, _owner: &Spatial) {
        let cfg: GameConfig = load_config(CONFIG_PATH.to_string());

        self.skeleton_ref = _owner.get_node("Skeleton")
            .expect("Skeleton node should exist");
        self.animation_player_ref = _owner.get_node("AnimationPlayer")
            .expect("AnimationPlayer node should exist");
        self.animation_tree_ref = _owner.get_node("AnimationTree")
            .expect("AnimationTree node should exist");

        self.gravity = cfg.gravity;
        self.player_speed = cfg.player_speed;

        godot_print!("{} is ready!", self.name);
    }

    // This function will be called in every frame
    #[export]
    unsafe fn _process(&self, _owner: &Spatial, delta: f64) {
        let input = Input::godot_singleton();

        let animation_tree_ref = unsafe { self.animation_tree_ref.assume_safe() };
        let animation_tree_ref = animation_tree_ref.cast::<AnimationTree>()
            .expect("Node should cast to AnimationTree");

        let player_transform = _owner.transform();
        let mut player_position = player_transform.origin;

        // animation_tree_ref.set("parameters/Iddle/blend_position", 2);

        if Input::is_action_pressed(&input, "ui_right") {
            player_position.x -= self.player_speed;
        }
        if Input::is_action_pressed(&input, "ui_left") {
            player_position.x += self.player_speed;
        }
        if Input::is_action_pressed(&input, "ui_down") {
            player_position.z -= self.player_speed;
        }
        if Input::is_action_pressed(&input, "ui_up") {
            player_position.z += self.player_speed;
            animation_tree_ref.set("parameters/Running", 0.2);
        }
        if Input::is_action_pressed(&input, "jump") {
            player_position.y += self.player_speed;
            animation_tree_ref.set("parameters/State/current", 2);
        }
        if Input::is_action_pressed(&input, "crouch") {
            player_position.y -= self.player_speed;
            animation_tree_ref.set("parameters/State/current", 1);
        }
        _owner.set_translation(player_position);
    }
}

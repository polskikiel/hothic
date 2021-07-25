use gdnative::api::*;
use gdnative::prelude::*;
use std::io;
use std::ops::Deref;

#[derive(NativeClass)]
#[inherit(Spatial)]
#[register_with(Self::register_builder)]
pub struct Player {
    name: String,
    player_init_vector: Vector3,
    player_init_zoom: f64,
    player_wrapper: Ref<Spatial>,
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
            name: "".to_string(),
            player_init_vector: Vector3::new(0.0, 0.0, 0.0),
            player_init_zoom: 0.0,
            player_wrapper: Spatial::new().into_shared(),
        }
    }

    // In order to make a method known to Godot, the #[export] attribute has to be used.
    // In Godot script-classes do not actually inherit the parent class.
    // Instead they are "attached" to the parent object, called the "owner".
    // The owner is passed to every single exposed method.
    #[export]
    unsafe fn _ready(&mut self, _owner: &Spatial) {
        self.name = "Player".to_string();

        // self.player_wrapper = unsafe {
        //     _owner.get_node_as::<Spatial>("PlayerWrapper").unwrap().claim().into()
        // };
        // self.player_init_vector = self.player_wrapper.into().transform();
        // self.player_init_zoom = self.player_wrapper.into().transform().length();
        // let player_transform = self.player_wrapper.into().transform();
        // let player_position = player_transform.origin;

        godot_print!("{} is ready!", self.name);
    }

    // This function will be called in every frame
    #[export]
    unsafe fn _process(&self, _owner: &Spatial, delta: f64) {
        let input = Input::godot_singleton();

        // let player_transform = self.player_wrapper.cast().into().transform();
        // let player_position = player_transform.origin;
        //
        // if Input::is_action_pressed(&input, "ui_right") {
        //     player_position.x += 1.0
        // }
        // if Input::is_action_pressed(&input, "ui_left") {
        //     player_position.x -= 1.0
        // }
        // if Input::is_action_pressed(&input, "ui_down") {
        //     player_position.y += 1.0
        // }
        // if Input::is_action_pressed(&input, "ui_up") {
        //     player_position.y -= 1.0
        // }
        //
        //
        // self.player_wrapper.set_transform(player_position);
    }
}

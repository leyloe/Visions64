use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_stuff::plugins::{
    camera::{self, CameraAction},
    player, window, world,
};
use leafwing_input_manager::plugin::InputManagerPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            InputManagerPlugin::<CameraAction>::default(),
            world::plugin,
            player::plugin,
            camera::plugin,
            window::plugin,
        ))
        .run();
}

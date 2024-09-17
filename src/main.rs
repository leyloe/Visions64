use avian3d::prelude::*;
use avian_interpolation3d::AvianInterpolationPlugin;
use bevy::prelude::*;

use bevy_stuff::{
    player::{camera, controls::PlayerAction, movement},
    window, world,
};
use leafwing_input_manager::plugin::InputManagerPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default().build().disable::<SyncPlugin>(),
            AvianInterpolationPlugin::default(),
            InputManagerPlugin::<PlayerAction>::default(),
            world::plugin,
            movement::plugin,
            camera::plugin,
            window::plugin,
        ))
        .run();
}

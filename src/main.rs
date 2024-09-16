use avian3d::prelude::*;
use avian_interpolation3d::AvianInterpolationPlugin;
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
            PhysicsPlugins::default().build().disable::<SyncPlugin>(),
            AvianInterpolationPlugin::default(),
            InputManagerPlugin::<CameraAction>::default(),
            world::plugin,
            player::plugin,
            camera::plugin,
            window::plugin,
        ))
        .run();
}

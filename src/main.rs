use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_stuff::systems::{r#move::camera_movement, startup::setup};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            bevy_fpc::FpcPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, camera_movement)
        .run();
}

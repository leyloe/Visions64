use bevy::{prelude::*, window::close_on_esc};
use bevy_rapier3d::prelude::*;
use bevy_stuff::systems::startup::setup;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            bevy_fpc::FpcPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        .run();
}

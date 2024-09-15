use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_stuff::plugins::{player, window, world};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            world::plugin,
            player::plugin,
            window::plugin,
        ))
        .run();
}

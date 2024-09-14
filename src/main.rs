use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_stuff::systems::{
    player::{player_move, spawn_camera},
    window::{close_on_esc, lock_mouse},
    world::{spawn_lights, spawn_world_model},
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, RapierPhysicsPlugin::<NoUserData>::default()))
        .add_systems(
            Startup,
            (spawn_camera, lock_mouse, spawn_world_model, spawn_lights),
        )
        .add_systems(FixedUpdate, player_move)
        .add_systems(Update, close_on_esc)
        .run();
}

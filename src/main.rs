use avian3d::prelude::*;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_stuff::systems::{
    player::{camera_move, player_move, spawn_player},
    window::toggle_mouse,
    world::{spawn_lights, spawn_world_model},
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, (spawn_player, spawn_world_model, spawn_lights))
        .add_systems(FixedUpdate, (player_move, camera_move))
        .add_systems(
            Update,
            (toggle_mouse.run_if(input_just_pressed(KeyCode::Escape)),),
        )
        .run();
}

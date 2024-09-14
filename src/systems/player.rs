use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use crate::{components::Player, constants::PLAYER_MOVEMENT_SPEED};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Player,
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.50, 0.0),
            ..default()
        },
    ));
}

pub fn player_move(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut delta = Vec3::default();
    let mut transform = query.single_mut();

    if keyboard_input.pressed(KeyCode::KeyA) {
        delta.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        delta.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        delta.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        delta.z += 1.0;
    }

    transform.translation.x += delta.x * PLAYER_MOVEMENT_SPEED * time.delta_seconds();

    transform.translation.z += delta.z * PLAYER_MOVEMENT_SPEED * time.delta_seconds();
}

pub fn camera_move(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = query.single_mut();
    for event in mouse_motion_events.read() {
        transform.rotation.x += event.delta.x;
        transform.rotation.y += event.delta.y;
    }
}

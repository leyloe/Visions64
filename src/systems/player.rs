use avian3d::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use crate::{
    components::Player,
    constants::{PITCH_LIMIT, PLAYER_CAMERA_SENSITIVITY, PLAYER_MOVEMENT_SPEED},
};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Player,
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
        Collider::capsule(0.5, 1.0),
        RigidBody::Dynamic,
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

    let movement = transform.rotation * delta * PLAYER_MOVEMENT_SPEED * time.delta_seconds();

    transform.translation += movement;
}

pub fn camera_move(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = query.single_mut();
    for event in mouse_motion_events.read() {
        let delta_yaw = -event.delta.x * PLAYER_CAMERA_SENSITIVITY.x;
        let delta_pitch = -event.delta.y * PLAYER_CAMERA_SENSITIVITY.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

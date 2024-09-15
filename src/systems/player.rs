use avian3d::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use crate::{
    components::Player,
    constants::{PITCH_LIMIT, PLAYER_CAMERA_SENSITIVITY, PLAYER_MOVEMENT_SPEED},
};

pub fn spawn_player(mut commands: Commands) {
    commands
        .spawn((
            Player,
            Transform::from_xyz(0.0, 1.0, 0.0),
            GlobalTransform::default(),
            LockedAxes::new()
                .lock_rotation_x()
                .lock_rotation_y()
                .lock_rotation_z(),
            Collider::capsule(0.5, 1.),
            RigidBody::Dynamic,
        ))
        .with_children(|parent| {
            parent.spawn(Camera3dBundle { ..default() });
        });
}

pub fn player_move(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    camera_query: Query<&Transform, (With<Camera>, Without<Player>)>,
    time: Res<Time>,
) {
    let mut delta = Vec3::default();

    let camera_transform = camera_query.single();

    let mut player_transform = player_query.single_mut();

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

    let movement = camera_transform.rotation * delta * PLAYER_MOVEMENT_SPEED * time.delta_seconds();

    player_transform.translation += movement;
}

pub fn camera_move(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera>>,
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

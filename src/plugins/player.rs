use avian3d::{
    math::{Scalar, Vector},
    prelude::*,
};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use crate::constants::{
    MAX_SLOPE_ANGLE, PITCH_LIMIT, PLAYER_CAMERA_SENSITIVITY, PLAYER_MOVEMENT_SPEED,
};

#[derive(Component)]
struct Player;

#[derive(Component)]
pub struct MaxSlopeAngle(Scalar);

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_player)
        .add_systems(FixedUpdate, (move_player, move_camera, update_grounded));
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn((
            Player,
            Transform::from_xyz(0.0, 1.0, 0.0),
            GlobalTransform::default(),
            LockedAxes::ROTATION_LOCKED,
            Collider::capsule(0.5, 1.),
            RigidBody::Dynamic,
            MaxSlopeAngle(MAX_SLOPE_ANGLE),
            Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            GravityScale(2.0),
        ))
        .with_children(|parent| {
            parent.spawn(Camera3dBundle { ..default() });
        });
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    camera_query: Query<&Transform, (With<Camera>, Without<Player>)>,
    time: Res<Time>,
) {
    let mut delta = Vec3::default();

    let camera_transform = camera_query.single();

    let mut player_transform = player_query.single_mut();

    if keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        delta.x -= 1.0;
    }
    if keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        delta.x += 1.0;
    }
    if keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
        delta.z -= 1.0;
    }
    if keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
        delta.z += 1.0;
    }

    let yaw_rotation = Quat::from_rotation_y(camera_transform.rotation.to_euler(EulerRot::YXZ).0);

    let movement = yaw_rotation * delta * PLAYER_MOVEMENT_SPEED * time.delta_seconds();

    player_transform.translation += movement;
}

fn move_camera(
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

fn update_grounded(
    mut commands: Commands,
    mut query: Query<(Entity, &ShapeHits, &Rotation, Option<&MaxSlopeAngle>), With<Player>>,
) {
    for (entity, hits, rotation, max_slope_angle) in &mut query {
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                (rotation * -hit.normal2).angle_between(Vector::Y).abs() <= angle.0
            } else {
                true
            }
        });

        println!("Entity: {:?}, Is Grounded: {:?}", entity, is_grounded);

        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

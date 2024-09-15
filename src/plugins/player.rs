use avian3d::{
    math::{AdjustPrecision as _, Quaternion, Scalar, Vector},
    prelude::*,
};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use crate::constants::{
    DAMPING, JUMP_IMPULSE, MAX_SLOPE_ANGLE, MOVEMENT_ACCELERATION, PITCH_LIMIT,
    PLAYER_CAMERA_SENSITIVITY, PLAYER_MOVEMENT_SPEED,
};

#[derive(Component)]
struct Player;

#[derive(Component)]
pub struct MaxSlopeAngle(Scalar);

#[derive(Component)]
pub struct JumpImpulse(Scalar);

#[derive(Component)]
pub struct MovementAcceleration(Scalar);

#[derive(Component)]
pub struct MovementDampingFactor(Scalar);

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_player)
        .add_systems(
            FixedUpdate,
            (
                move_player,
                update_grounded,
                apply_movement_damping,
                player_jump,
            )
                .chain(),
        )
        .add_systems(Update, move_camera);
}

fn spawn_player(mut commands: Commands) {
    let collider = Collider::capsule(0.5, 1.);

    let mut caster_shape = collider.clone();
    caster_shape.set_scale(Vector::ONE * 0.99, 10);

    let ground_caster = ShapeCaster::new(
        caster_shape,
        Vector::ZERO,
        Quaternion::default(),
        Dir3::NEG_Y,
    )
    .with_max_time_of_impact(0.2);

    commands
        .spawn((
            Player,
            Transform::from_xyz(0.0, 1.0, 0.0),
            GlobalTransform::default(),
            LockedAxes::ROTATION_LOCKED,
            collider,
            ground_caster,
            RigidBody::Dynamic,
            MaxSlopeAngle(MAX_SLOPE_ANGLE),
            JumpImpulse(JUMP_IMPULSE),
            MovementAcceleration(MOVEMENT_ACCELERATION),
            MovementDampingFactor(DAMPING),
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
    mut player_query: Query<(&mut LinearVelocity, &MovementAcceleration), With<Player>>,
    camera_query: Query<&Transform, (With<Camera>, Without<Player>)>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds_f64().adjust_precision();

    let mut delta = Vec3::default();

    let camera_transform = camera_query.single();

    let (mut linear_velocity, movement_acceleration) = player_query.single_mut();

    if keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        delta.x -= 1.0;
    }
    if keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        delta.x += 1.0;
    }
    if keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
        delta.z -= 1.0;
    }
    if keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
        delta.z += 1.0;
    }

    let forward = camera_transform.forward();
    let right = camera_transform.right();

    let movement_direction = (right * delta.x + forward * delta.z).normalize_or_zero();

    linear_velocity.x +=
        movement_direction.x * movement_acceleration.0 * delta_time * PLAYER_MOVEMENT_SPEED;
    linear_velocity.z +=
        movement_direction.z * movement_acceleration.0 * delta_time * PLAYER_MOVEMENT_SPEED;
}

fn player_jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(Option<&Grounded>, &mut LinearVelocity, &JumpImpulse), With<Player>>,
) {
    let (grounded, mut linear_velocity, jump_impulse) = player_query.single_mut();

    if keyboard_input.pressed(KeyCode::Space) && grounded.is_some() {
        linear_velocity.y = jump_impulse.0;
    }
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

        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

fn apply_movement_damping(
    mut query: Query<(&MovementDampingFactor, &mut LinearVelocity), With<Player>>,
) {
    for (damping_factor, mut linear_velocity) in &mut query {
        linear_velocity.x *= damping_factor.0;
        linear_velocity.z *= damping_factor.0;
    }
}

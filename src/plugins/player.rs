use avian3d::{
    math::{AdjustPrecision as _, Quaternion, Scalar, Vector},
    prelude::*,
};
use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, Actionlike};

use crate::constants::{
    DAMPING, GRAVITY_SCALE, JUMP_IMPULSE, MAX_SLOPE_ANGLE, MOVEMENT_ACCELERATION,
    PLAYER_MOVEMENT_SPEED,
};

use super::camera::PlayerCamera;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MaxSlopeAngle(Scalar);

#[derive(Component)]
pub struct JumpImpulse(Scalar);

#[derive(Component)]
pub struct MovementAcceleration(Scalar);

#[derive(Component)]
pub struct MovementDampingFactor(Scalar);

#[derive(Component)]
pub struct Grounded;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    #[actionlike(DualAxis)]
    RotateCamera,
    #[actionlike(DualAxis)]
    Move,
    Jump,
}

impl PlayerAction {
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map.insert_dual_axis(PlayerAction::Move, KeyboardVirtualDPad::WASD);

        input_map.insert_dual_axis(PlayerAction::Move, KeyboardVirtualDPad::ARROW_KEYS);

        input_map.insert(PlayerAction::Jump, KeyCode::Space);

        input_map.insert_dual_axis(PlayerAction::Move, GamepadStick::LEFT);

        input_map.insert(PlayerAction::Jump, GamepadButtonType::South);

        input_map.insert_dual_axis(PlayerAction::RotateCamera, GamepadStick::RIGHT);

        input_map.insert_dual_axis(PlayerAction::RotateCamera, MouseMove::default());

        input_map
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_player).add_systems(
        FixedUpdate,
        (
            move_player,
            update_grounded,
            apply_movement_damping,
            player_jump,
        )
            .chain(),
    );
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

    commands.spawn((
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
        InputManagerBundle::with_map(PlayerAction::default_input_map()),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        GravityScale(GRAVITY_SCALE),
    ));
}

fn move_player(
    mut player_query: Query<
        (
            &mut LinearVelocity,
            &MovementAcceleration,
            &ActionState<PlayerAction>,
        ),
        With<Player>,
    >,
    camera_query: Query<&Transform, (With<PlayerCamera>, Without<Player>)>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds_f64().adjust_precision();

    let camera_transform = camera_query.single();

    let (mut linear_velocity, movement_acceleration, action_state) = player_query.single_mut();

    let delta = action_state.axis_pair(&PlayerAction::Move);

    let mut forward: Vec3 = camera_transform.forward().into();
    forward.y = 0.0;
    forward = forward.normalize_or_zero();

    let right = camera_transform.right();

    let movement_direction = (right * delta.x + forward * delta.y).normalize_or_zero();

    linear_velocity.x +=
        movement_direction.x * movement_acceleration.0 * delta_time * PLAYER_MOVEMENT_SPEED;
    linear_velocity.z +=
        movement_direction.z * movement_acceleration.0 * delta_time * PLAYER_MOVEMENT_SPEED
}

fn player_jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(Option<&Grounded>, &mut LinearVelocity, &JumpImpulse), With<Player>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        let (grounded, mut linear_velocity, jump_impulse) = player_query.single_mut();
        if grounded.is_some() {
            linear_velocity.y = jump_impulse.0;
        }
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

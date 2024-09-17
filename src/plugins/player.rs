use avian3d::{
    math::{Quaternion, Vector},
    prelude::*,
};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::constants::{
    DAMPING, GRAVITY_SCALE, JUMP_IMPULSE, MAX_SLOPE_ANGLE, MOVEMENT_ACCELERATION,
    PLAYER_MOVEMENT_SPEED,
};

use super::{camera::PlayerCamera, controls::PlayerAction};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Grounded;

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
        RigidBody::Dynamic,
        InputManagerBundle::with_map(PlayerAction::default_input_map()),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        GravityScale(GRAVITY_SCALE),
        collider,
        ground_caster,
    ));
}

fn move_player(
    mut player_query: Query<(&mut LinearVelocity, &ActionState<PlayerAction>), With<Player>>,
    camera_query: Query<&Transform, (With<PlayerCamera>, Without<Player>)>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    let camera_transform = camera_query.single();

    let (mut linear_velocity, action_state) = player_query.single_mut();

    let delta = action_state.axis_pair(&PlayerAction::Move);

    let mut forward: Vec3 = camera_transform.forward().into();
    forward.y = 0.0;
    forward = forward.normalize_or_zero();

    let right = camera_transform.right();

    let movement_direction = (right * delta.x + forward * delta.y).normalize_or_zero();

    linear_velocity.x +=
        movement_direction.x * MOVEMENT_ACCELERATION * delta_time * PLAYER_MOVEMENT_SPEED;
    linear_velocity.z +=
        movement_direction.z * MOVEMENT_ACCELERATION * delta_time * PLAYER_MOVEMENT_SPEED
}

type PlayerJumpQuery<'a> = (
    &'a ActionState<PlayerAction>,
    Option<&'a Grounded>,
    &'a mut LinearVelocity,
);

fn player_jump(mut player_query: Query<PlayerJumpQuery, With<Player>>) {
    let (action_state, grounded, mut linear_velocity) = player_query.single_mut();

    if action_state.pressed(&PlayerAction::Jump) && grounded.is_some() {
        linear_velocity.y = JUMP_IMPULSE;
    }
}

fn update_grounded(
    mut commands: Commands,
    mut query: Query<(Entity, &ShapeHits, &Rotation), With<Player>>,
) {
    for (entity, hits, rotation) in &mut query {
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = MAX_SLOPE_ANGLE {
                (rotation * -hit.normal2).angle_between(Vector::Y).abs() <= angle
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

fn apply_movement_damping(mut query: Query<&mut LinearVelocity, With<Player>>) {
    for mut linear_velocity in &mut query {
        linear_velocity.x *= DAMPING;
        linear_velocity.z *= DAMPING;
    }
}

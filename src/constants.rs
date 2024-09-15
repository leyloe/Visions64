use bevy::math::Vec2;
use std::f32::consts::{FRAC_PI_2, PI};

pub const PLAYER_MOVEMENT_SPEED: f32 = 5.0;
pub const PLAYER_CAMERA_SENSITIVITY: Vec2 = Vec2::new(0.003, 0.002);
pub const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
pub const MAX_SLOPE_ANGLE: f32 = PI * 0.45;

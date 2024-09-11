use bevy::prelude::*;
use bevy_fpc::Player;

pub fn camera_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
}

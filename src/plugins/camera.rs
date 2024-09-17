use bevy::{app::RunFixedMainLoop, prelude::*, time::run_fixed_main_schedule};
use leafwing_input_manager::{prelude::*, InputManagerBundle};

use crate::constants::{FIELD_OF_VIEW, PITCH_LIMIT, PLAYER_CAMERA_SENSITIVITY};

use super::player::{Player, PlayerAction};

#[derive(Component)]
pub struct PlayerCamera;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera)
        .add_systems(
            RunFixedMainLoop,
            rotate_camera.before(run_fixed_main_schedule),
        )
        .add_systems(Update, follow_player);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        PlayerCamera,
        Camera3dBundle {
            projection: PerspectiveProjection {
                fov: FIELD_OF_VIEW.to_radians(),
                ..default()
            }
            .into(),
            ..default()
        },
        InputManagerBundle::with_map(PlayerAction::default_input_map()),
    ));
}

fn follow_player(
    mut q_camera: Query<&mut Transform, With<PlayerCamera>>,
    q_player: Query<&Transform, (With<Player>, Without<PlayerCamera>)>,
) {
    let Ok(player_transform) = q_player.get_single() else {
        return;
    };
    let Ok(mut camera_transform) = q_camera.get_single_mut() else {
        return;
    };
    let height_offset = 0.5;
    camera_transform.translation =
        player_transform.translation + player_transform.up() * height_offset;
}

fn rotate_camera(mut character_query: Query<(&mut Transform, &ActionState<PlayerAction>)>) {
    for (mut transform, action_state) in &mut character_query {
        let delta = action_state.axis_pair(&PlayerAction::RotateCamera);
        let delta_yaw = -delta.x * PLAYER_CAMERA_SENSITIVITY.x;
        let delta_pitch = -delta.y * PLAYER_CAMERA_SENSITIVITY.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

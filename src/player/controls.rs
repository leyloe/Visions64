use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, Actionlike};

use crate::constants::{PLAYER_CAMERA_CONTROLLER_SENSITIVITY, PLAYER_CAMERA_MOUSE_SENSITIVITY};

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

        input_map.insert_dual_axis(
            PlayerAction::RotateCamera,
            MouseMove::default()
                .sensitivity_x(PLAYER_CAMERA_MOUSE_SENSITIVITY.x)
                .sensitivity_y(PLAYER_CAMERA_MOUSE_SENSITIVITY.y),
        );

        input_map.insert_dual_axis(
            PlayerAction::RotateCamera,
            GamepadStick::RIGHT
                .sensitivity_x(PLAYER_CAMERA_CONTROLLER_SENSITIVITY.x)
                .sensitivity_y(PLAYER_CAMERA_CONTROLLER_SENSITIVITY.y)
                .with_deadzone_symmetric(0.2)
                .inverted_y(),
        );

        input_map.insert_dual_axis(PlayerAction::Move, KeyboardVirtualDPad::WASD);

        input_map.insert_dual_axis(PlayerAction::Move, KeyboardVirtualDPad::ARROW_KEYS);

        input_map.insert_dual_axis(
            PlayerAction::Move,
            GamepadStick::LEFT.with_deadzone_symmetric(0.2),
        );

        input_map.insert(PlayerAction::Jump, KeyCode::Space);

        input_map.insert(PlayerAction::Jump, GamepadButtonType::South);

        input_map
    }
}

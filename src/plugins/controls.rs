use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, Actionlike};

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

        input_map.insert_dual_axis(PlayerAction::RotateCamera, MouseMove::default());

        input_map.insert_dual_axis(PlayerAction::RotateCamera, GamepadStick::RIGHT);

        input_map.insert_dual_axis(PlayerAction::Move, KeyboardVirtualDPad::WASD);

        input_map.insert_dual_axis(PlayerAction::Move, KeyboardVirtualDPad::ARROW_KEYS);

        input_map.insert_dual_axis(PlayerAction::Move, GamepadStick::LEFT);

        input_map.insert(PlayerAction::Jump, KeyCode::Space);

        input_map.insert(PlayerAction::Jump, GamepadButtonType::South);

        input_map
    }
}

use bevy::{
    input::common_conditions::input_just_pressed,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow, WindowMode},
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            toggle_mouse.run_if(input_just_pressed(KeyCode::Escape)),
            toggle_fullscreen.run_if(input_just_pressed(KeyCode::F11)),
        ),
    );
}

fn toggle_mouse(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    for mut window in &mut window {
        match window.cursor.grab_mode {
            CursorGrabMode::None => {
                window.cursor.grab_mode = CursorGrabMode::Locked;
                window.cursor.visible = false;
            }
            CursorGrabMode::Confined | CursorGrabMode::Locked => {
                window.cursor.grab_mode = CursorGrabMode::None;
                window.cursor.visible = true;
            }
        }
    }
}

fn toggle_fullscreen(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    for mut window in &mut window {
        match window.mode {
            WindowMode::Fullscreen => window.mode = WindowMode::Windowed,
            _ => window.mode = WindowMode::Fullscreen,
        }
    }
}

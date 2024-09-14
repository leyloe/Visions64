use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

pub fn lock_mouse(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    for mut window in &mut window {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }
}

pub fn toggle_mouse(mut window: Query<&mut Window, With<PrimaryWindow>>) {
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

use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use std::f32::consts::PI;

use crate::{EditorState, GameState, Player};

#[derive(Default)]
pub struct MouseState {
    pub press_coords: Vec<Vec2>,
}

impl Resource for MouseState {}

pub fn keyboard_input(
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Player>,
    time: Res<'_, Time<Real>>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    editor_state: Res<State<EditorState>>,
    mut next_editor_state: ResMut<NextState<EditorState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }

    if keyboard_input.just_pressed(KeyCode::AltLeft) {
        match game_state.get() {
            GameState::InGame => {
                next_game_state.set(GameState::InEditor);
            }
            GameState::InEditor => {
                next_game_state.set(GameState::InGame);
                let mut primary_window = window.single_mut();

                if primary_window.cursor.grab_mode != CursorGrabMode::Locked {
                    primary_window.cursor.grab_mode = CursorGrabMode::Locked;
                    primary_window.cursor.visible = false;
                }
            }
        }
    }

    if keyboard_input.just_pressed(KeyCode::Tab) {
        lock_cursor(&mut window);
    }

    if keyboard_input.just_pressed(KeyCode::KeyM) {
        match game_state.get() {
            GameState::InEditor => match editor_state.get() {
                EditorState::World => {
                    next_editor_state.set(EditorState::Map);
                }
                EditorState::Map => {
                    next_editor_state.set(EditorState::World);
                }
            },
            GameState::InGame => {}
        }
    }

    for mut player in query.iter_mut() {
        let speed = 50.;

        let mut movement = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            let yaw_offset = player.yaw - PI / 2.0;
            movement += Vec3::new(yaw_offset.cos(), 0., yaw_offset.sin());
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            movement -= Vec3::new(player.yaw.cos(), 0., player.yaw.sin());
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            let yaw_offset = player.yaw + PI / 2.0;
            movement += Vec3::new(yaw_offset.cos(), 0., yaw_offset.sin());
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            movement += Vec3::new(player.yaw.cos(), 0., player.yaw.sin());
        }
        if keyboard_input.pressed(KeyCode::KeyE) {
            movement += Vec3::new(0., 1., 0.);
        }
        if keyboard_input.pressed(KeyCode::KeyQ) {
            movement -= Vec3::new(0., 1., 0.);
        }

        movement = movement.normalize_or_zero() * speed * time.delta_seconds();

        player.x += movement.x;
        player.y += movement.y;
        player.z += movement.z;
    }
}

pub fn mouse_input(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut mouse_state: ResMut<MouseState>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut query: Query<&mut Player>,
) {
    for event in mouse_motion_events.read() {
        let primary_window = window_query.single_mut();

        if primary_window.cursor.grab_mode == CursorGrabMode::Locked {
            let delta = event.delta;
            for mut player in query.iter_mut() {
                let sensitivity = 0.005;

                player.yaw += delta.x * sensitivity;
                player.yaw = player.yaw.rem_euclid(2.0 * PI);
                player.pitch -= delta.y * sensitivity;
                player.pitch = player.pitch.clamp(-PI / 2.0, PI / 2.0);
            }
        }
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        mouse_state.press_coords.clear();

        let window = window_query.single_mut();
        let _window_pos = window.cursor_position().unwrap();
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        let window = window_query.single_mut();

        if window.cursor_position() == None {
            return;
        }

        let _window_pos = window.cursor_position().unwrap();
    }
}

pub fn lock_cursor(window: &mut Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = window.single_mut();

    if primary_window.cursor.grab_mode == CursorGrabMode::Locked {
        primary_window.cursor.grab_mode = CursorGrabMode::None;
        primary_window.cursor.visible = true;
    } else {
        // for a game that doesn't use the cursor (like a shooter):
        // use `Locked` mode to keep the cursor in one place
        primary_window.cursor.grab_mode = CursorGrabMode::Locked;

        // also hide the cursor
        primary_window.cursor.visible = false;
    }
}

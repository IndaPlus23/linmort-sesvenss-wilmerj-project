use crate::Player;
use bevy::{input::mouse::MouseMotion, prelude::*};
use std::f32::consts::PI;

#[derive(Default)]
pub struct MouseState {
    pub press_coords: Vec<Vec2>,
}

impl Resource for MouseState {}

pub fn keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Player>,
    time: Res<'_, Time<Real>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
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
    mut windows: Query<&mut Window>,
    mut query: Query<&mut Player>,
) {
    for event in mouse_motion_events.read() {
        let delta = event.delta;
        for mut player in query.iter_mut() {
            let sensitivity = 0.005;

            player.yaw += delta.x * sensitivity;
            player.yaw = player.yaw.rem_euclid(2.0 * PI);
            player.pitch -= delta.y * sensitivity;
            player.pitch = player.pitch.clamp(-PI / 2.0, PI / 2.0);
        }
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        mouse_state.press_coords.clear();

        let window = windows.single_mut();
        let window_pos = window.cursor_position().unwrap();

        let mut cursor_world_pos = window_pos;
        for player in query.iter_mut() {
            cursor_world_pos.x -= window.width() / 2.0 - player.x;
            cursor_world_pos.y -= window.height() / 2.0 + player.y;
            cursor_world_pos.y *= -1.;
        }

        cursor_world_pos.x = (cursor_world_pos.x / 25.0).round() * 25.0;
        cursor_world_pos.y = (cursor_world_pos.y / 25.0).round() * 25.0;

        mouse_state.press_coords.push(cursor_world_pos);
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        let window = windows.single_mut();
        let window_pos = window.cursor_position().unwrap();

        let mut cursor_world_pos = window_pos;

        for player in query.iter_mut() {
            cursor_world_pos.x -= window.width() / 2.0 - player.x;
            cursor_world_pos.y -= window.height() / 2.0 + player.y;
            cursor_world_pos.y *= -1.;
        }

        cursor_world_pos.x = (cursor_world_pos.x / 25.0).round() * 25.0;
        cursor_world_pos.y = (cursor_world_pos.y / 25.0).round() * 25.0;

        let _starting_position = mouse_state.press_coords.last().unwrap().clone();
    }
}

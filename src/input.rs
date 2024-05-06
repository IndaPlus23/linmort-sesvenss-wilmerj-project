use crate::collision_detection::{wall_collision, floor_collision};

use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use std::f32::consts::PI;

use crate::{map::Map, Player, Wall, floor::Floor};

#[derive(Default)]
pub struct MouseState {
    pub press_coords: Vec<Vec2>,
}

impl Resource for MouseState {}

pub fn keyboard_input(
    map_query: Query<&mut Map>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Player>,
    time: Res<'_, Time<Real>>,
    mut wall_query: Query<&mut Wall>,
    mut floor_query: Query<&mut Floor>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }

    if keyboard_input.just_pressed(KeyCode::Enter) {
        for map in map_query.iter() {
            map.save();
            println!("saved");
        }
    }

    if keyboard_input.just_pressed(KeyCode::Tab) {
        let mut primary_window = window_query.single_mut();

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

    for mut player in query.iter_mut() {
        let mut speed = 1./3.;

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
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            speed = 1.0
        }

        // GRAVITY + JUMPING
        if keyboard_input.pressed(KeyCode::Space) { // jump

            if player.gravity < 30. {
                movement += Vec3::new(0., 2., 0.); // add y velocity
            }
            player.gravity += 1.0; // sort of a "timer" that counts how long the player jumped
        }

        // first part checks if player hit "rock bottom" so that they dont fall forever, then checks if the player is currently falling
        if player.y + movement.y - player.height < -3. && movement.y <= 0. { 
            movement.y = 0.
        } else {
            movement.y -= 1. // If the player is in the air and not fallint -> start falling
        }

        // CHECKS EVERY FLOOR FOR COLLISION
        for floor in floor_query.iter_mut() {
            floor_collision(&floor, &mut movement, &mut player);
        }

        // CHECKS EVERY WALL FOR COLLISION
        for wall in wall_query.iter_mut() {
            wall_collision(&wall, &mut movement, &mut player)
        }

        // when player has landed on the ground reset the "timer"
        if player.gravity > 0. && movement.y == 0.{
            player.gravity = 0.0;
        }          
        
        movement = movement * speed;//movement.normalize_or_zero() * speed * time.delta_seconds();

        
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
use crate::{floor, Player};
use crate::wall::Wall; // wall_collision
use crate::floor::Floor; // wall_collision
use crate::CustomMaterial; // wall_collision
use bevy::{prelude::*, render::mesh::Mesh, sprite::Mesh2dHandle}; // wall_collision
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy::{input::mouse::MouseMotion, prelude::*};
use std::f32::consts::PI;

#[derive(Default)]
pub struct MouseState {
    pub press_coords: Vec<Vec2>,
}

impl Resource for MouseState {}

pub fn keyboard_input(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Player>,
    time: Res<'_, Time<Real>>,

    // ADDED wall_query TO CHECK IF POSITION OF PLAYER IS THE SAME AS WALL/FLOOR FOR BASIC WALL COLLISION
    mut wall_query: Query<(
        &mut Wall,
        &mut Transform,
        &mut Mesh2dHandle,
        &mut Handle<CustomMaterial>,
    )>,
    mut floor_query: Query<
        (
            &mut Floor,
            &mut Transform,
            &mut Mesh2dHandle,
            &mut Handle<CustomMaterial>,
        ),
        Without<Wall>,
    >,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
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

    if keyboard_input.just_pressed(KeyCode::KeyN) {

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

        
        

        // CHECKS EVERY WALL FOR COLLISION
        for (wall, _transform, _mesh2dhandle, _material_handle) in wall_query.iter_mut() {
            wall_collision(&wall, &mut movement, &player);
        }
        // CHECKS EVERY FLOOR FOR COLLISION
        for (floor, _transform, _mesh2dhandle, _material_handle) in floor_query.iter_mut() {
            floor_collision(&floor, &mut movement, &player);
        }


        player.x += movement.x;
        player.y += movement.y;
        player.z += movement.z;
    }
}

/*
    WALL COLLISION

    CHECK IF 

    player.x + movement.x
    player.y + movement.y
    player.z + movement.z

    IS THE SAME VALUE AS WALL POSITION + PADDING

    IF SAME SET movement.z TO 0

    TODO: 
    clean up code 
    make sure collision works with floor and ceiling (when added)

*/

pub fn wall_collision(wall: &Mut<'_, Wall>, movement: &mut bevy::prelude::Vec3, player: &bevy::prelude::Mut<'_, Player>) {
    let padding = 1.5;

    let mut x_hit = false;
    let mut y_hit = false;
    let mut z_hit = false;
    
    // CHECK IF PLAYER HITS WALL IN x DIRECTION
    let position_x = player.x + movement.x;
    let wall_start_x = wall.start.position.x - padding;
    let wall_end_x = wall.end.position.x + padding;

    // YOU CAN ADD A WALL WHERE THE BEGINING IS SMALLER OR BIGGER THAN THE END, THIS IF/ELSE MAKES SURE WALL COLLISION STILL WORKS THEN
    if wall_start_x < wall_end_x {
        if position_x > wall_start_x && position_x < wall_end_x {
            x_hit = true
        }
    } else {
        if position_x < wall_start_x && position_x > wall_end_x {
            x_hit = true
        }
    }

    // CHECK IF PLAYER HITS WALL IN y DIRECTION
    let position_y = player.y + movement.y;
    let wall_start_y = wall.start.position.y - padding;
    let wall_end_y = wall.end.position.y + wall.height + padding;
    
    // YOU CAN ADD A WALL WHERE THE BEGINING IS SMALLER OR BIGGER THAN THE END, THIS IF/ELSE MAKES SURE WALL COLLISION STILL WORKS THEN
    if wall_start_y < wall_end_y {
        if position_y > wall_start_y && position_y < wall_end_y {
            y_hit = true
        }
    } else {
        if position_y < wall_start_y && position_y > wall_end_y {
            y_hit = true
        }
    }
    
    // CHECK IF PLAYER HITS WALL IN z DIRECTION
    let position_z = player.z + movement.z;
    let wall_start_z = wall.start.position.z - padding;
    let wall_end_z = wall.end.position.z + padding;

    // YOU CAN ADD A WALL WHERE THE BEGINING IS SMALLER OR BIGGER THAN THE END, THIS IF/ELSE MAKES SURE WALL COLLISION STILL WORKS THEN
    if wall_start_z < wall_end_z {
        if position_z > wall_start_z && position_z < wall_end_z {
            z_hit = true;
        }
    } else {
        if position_z < wall_start_z && position_z > wall_end_z {
            z_hit = true;
        }
    }

    if x_hit && y_hit && z_hit {
        
        // logic so that a player can "slide" against the wall
        if wall.start.position.x - wall.end.position.x == 0. {
            movement[0] = 0.
        }
        if wall.start.position.y - wall.end.position.y == 0. {
            movement[1] = 0.
        }
        if wall.start.position.z - wall.end.position.z == 0. {
            movement[2] = 0.
        }
    }
}

pub fn floor_collision(floor: &Mut<'_, Floor>, movement: &mut bevy::prelude::Vec3, player: &bevy::prelude::Mut<'_, Player>) {
    let padding = 1.0;

    let mut x_hit = false;
    let mut y_hit = false;
    let mut z_hit = false;
/* 
    floor.a.position.x

    // positiv X åt höger
    if player.x + movement.x > wall.start.position.x - padding && player.x + movement.x < wall.end.position.x + padding {
        x_hit = true
    }
    //positiv Y uppåt
    if player.y + movement.y > wall.start.position.y - padding && player.y + movement.y < wall.end.position.y + wall.height + padding {
        y_hit = true
    }
    // -Z i framåtriktningen
    if player.z + movement.z > wall.start.position.z - padding && player.z + movement.z < wall.end.position.z + padding {
        z_hit = true;
    }

    // DEBUGING
    //println!("START: {:?} END: {:?} PLAYER: ({:?}, {:?}, {:?} HIT: {:?} {:?} {:?})",wall.start, wall.end, player.x, player.y, player.z, x_hit, y_hit, z_hit);


    if x_hit && y_hit && z_hit {
        
        // logic so that a player can "slide" against the wall
        if wall.start.position.x - wall.end.position.x == 0. {
            movement[0] = 0.
        }
        if wall.start.position.y - wall.end.position.y == 0. {
            movement[1] = 0.
        }
        if wall.start.position.z - wall.end.position.z == 0. {
            movement[2] = 0.
        }
    } */
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
        let window = window_query.single_mut();
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

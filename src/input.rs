use crate::{floor, Player};
use crate::wall::Wall; // wall_collision
use crate::floor::Floor; // wall_collision
use crate::CustomMaterial; 
use bevy::core_pipeline::bloom::BloomPlugin;// wall_collision
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

    KNOWN BUGs:

    player can get stuck in the wall if they walk in towards the edge of a wall, 
    you can still walks alongside the wall but not away from it. 

    At the edge of floor triangels you can get throught the floor

    TODO: 
    fix bugs
    clean up code 
*/

pub fn wall_collision(wall: &Mut<'_, Wall>, movement: &mut bevy::prelude::Vec3, player: &bevy::prelude::Mut<'_, Player>) {
    let padding = 1.5;

    // FIND WHICH WAY THE WALL IS FACING ASUMING THAT ALL WALLS ARE FLAT
    
    if wall.start.position.x != wall.end.position.x {
        // WALL IS POINTING IN z DIRECTION
        // CHECK IF PLAYER IS IN SAME (x,y) AS WALL
        if is_inside_rectangle(wall.start.position.x, wall.start.position.y, wall.end.position.x, wall.end.position.y + wall.height, player.x + movement.x, player.y + movement.y) {
            // CHECK IF PLAYER HITS WALL IN z DIRECTION
            let position_z = player.z + movement.z;
            let wall_start_z = wall.start.position.z - padding;
            let wall_end_z = wall.end.position.z + padding;

            // CHECK IF THE PLAYER IS GOING TO MOVE THROUGH THE WALL
            if position_z > wall_start_z &&  position_z < wall_end_z {
                movement[2] = 0.
            }
        }
    }
    else if wall.start.position.z != wall.end.position.z {
        // WALL IS POINTING IN x DIRECTION
        // CHECK IF PLAYER IS IN SAME (z,y) AS WALL
        if is_inside_rectangle(wall.start.position.z, wall.start.position.y, wall.end.position.z, wall.end.position.y + wall.height, player.z + movement.z, player.y + movement.y) {
            // CHECK IF PLAYER HITS WALL IN x DIRECTION
            let position_x = player.x + movement.x;
            let wall_start_x = wall.start.position.x - padding;
            let wall_end_x = wall.end.position.x + padding;

            // CHECK IF THE PLAYER IS GOING TO MOVE THROUGH THE WALL
            if position_x > wall_start_x &&  position_x < wall_end_x {
                movement[0] = 0.
            }
        }
    }
}

// function to find if given point
// lies inside a given rectangle or not.
fn is_inside_rectangle(x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) -> bool {
    let mut check_x = false;
    let mut check_y = false;

    if x1 < x2 {
        if x > x1 && x < x2 {
            check_x = true
        }
    } else {
        if x < x1 && x > x2 {
            check_x = true
        }
    }
    if y1 < y2 {
        if y > y1 && y < y2 {
            check_y = true
        }
    } else {
        if y < y1 && y > y2 {
            check_y = true
        }
    }

    if check_x && check_y {
        return true;
    }
    return false; 
    /* if x > x1 && x < x2 && y > y1 && y < y2 {
        return true;
    }      
    return false; */
}

pub fn floor_collision(floor: &Mut<'_, Floor>, movement: &mut bevy::prelude::Vec3, player: &bevy::prelude::Mut<'_, Player>) {
    let padding = 1.0;

    //println!("{:?} {:?} {:?}", floor.a.position, floor.b.position, floor.c.position);

    let position_x = player.x + movement.x;
    let position_z = player.z + movement.z;

    // BEGIN BY CHECKING IF THE PLAYER IS ON THE FLOOR
    if is_inside_triangle(floor.a.position.x, floor.a.position.z, floor.b.position.x, floor.b.position.z, floor.c.position.x, floor.c.position.z, position_x, position_z) {
        
        // CHECK IF PLAYER HITS FLOOR IN y DIRECTION
        // ASUMES THAT ALL y VALUES ARE THE SAME AND THAT THE TRIANGLE IS ALWAYS FLAT
        let position_y = player.y + movement.y;
        let wall_start_y = floor.a.position.y - padding;
        let wall_end_y = floor.a.position.y + padding;

        if position_y > wall_start_y &&  position_y < wall_end_y {
            movement[1] = 0.
        }
        
    }
    
}

/* A function to check whether point P(x, y) lies inside the triangle formed 
   by A(x1, y1), B(x2, y2) and C(x3, y3) 
   shamelessly stolen from: https://www.geeksforgeeks.org/check-whether-a-given-point-lies-inside-a-triangle-or-not/
*/
fn is_inside_triangle(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, x: f32, y: f32) -> bool   {   
    /* Calculate area of triangle ABC */
    let entire_area: f32 = area(x1, y1, x2, y2, x3, y3);
    
    /* Calculate area of triangle PBC */ 
    let area_1: f32 = area(x, y, x2, y2, x3, y3);
    
    /* Calculate area of triangle PAC */ 
    let area_2: f32 = area(x1, y1, x, y, x3, y3);
    
    /* Calculate area of triangle PAB */  
    let area_3: f32 = area(x1, y1, x2, y2, x, y);
    
    /* Check if sum of A1, A2 and A3 is same as A */
    return entire_area == area_1 + area_2 + area_3;
}

/* A utility function to calculate area of triangle formed by (x1, y1), 
   (x2, y2) and (x3, y3) */
fn area(x1: f32,  y1: f32,  x2: f32,  y2: f32,  x3: f32,  y3: f32) -> f32   {
    return ((x1*(y2-y3) + x2*(y3-y1)+ x3*(y1-y2))/2.0).abs();
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

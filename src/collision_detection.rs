use std::f32::consts::PI;

use bevy::prelude::*;
use crate::wall::Wall;
use crate::floor::Floor;
use crate::Player;
/*
    WALL COLLISION

    KNOWN BUGs:

    1. player can get stuck in the wall if they walk in towards the edge of a wall, 
    you can still walks alongside the wall but not away from it. 
    2. wall collision does not work where 2 walls connect

    TODO: 
    fix bugs
    clean up code 

    // TODO: 
    // walk up stairs (maybe done)

*/



pub fn wall_collision(wall: &Mut<'_, Wall>, movement: &mut bevy::prelude::Vec3, player: &mut bevy::prelude::Mut<'_, Player>) {

    let player_vec: [f32; 3] = [
        player.x + movement.x,
        player.y + movement.y,
        player.z + movement.z,
        ];

    if check_if_wall(wall, player_vec, player) {
        
        // wall vector. if player hits wall it should move alongside the wall aka wall vector
        let wall_vector: Vec3 = Vec3::new(
            wall.start.position.x - wall.end.position.x, 
            0., 
            wall.start.position.z - wall.end.position.z
        ).normalize_or_zero();

        // calculate angle between wall_vector and movement
        let angle = movement.angle_between(wall_vector);

        // walking up stairs
        if player.height / 5. >= wall.height {
            player.y += wall.height + 2.;
        } 
        else if  PI/2. - 0.1 < angle && angle < PI/2. + 0.1 {
            movement.x = 0.;
            movement.z = 0.;
        }
        else if angle < PI/2. {
            movement.x = wall_vector.x;
            movement.z = wall_vector.z;
        }
        else if angle > PI/2. {
            movement.x = -wall_vector.x;
            movement.z = -wall_vector.z;
        } 
    }
}

fn check_if_wall(wall: &Mut<'_, Wall>, player_vec: [f32; 3], player: &mut bevy::prelude::Mut<'_, Player>) -> bool {

    
    
    let wall_1: Vec3 = wall.start.position;
    let wall_2: Vec3 = wall.end.position;

    // wall collision
    let vec1: [f32; 3] = [player_vec[0] - wall_1[0], player_vec[1] - wall_1[1], player_vec[2] - wall_1[2]];
    let vec2: [f32; 3] = [player_vec[0] - wall_2[0], player_vec[1] - wall_2[1], player_vec[2] - wall_2[2]];
    let vec3: [f32; 3] = [wall_1[0] - wall_2[0], wall_1[1] - wall_2[1], wall_1[2] - wall_2[2]];

    let distance_1: f32 = (vec1[0] * vec1[0] + vec1[1] * vec1[1] + vec1[2] * vec1[2]).sqrt();
    let distance_2: f32 = (vec2[0] * vec2[0] + vec2[1] * vec2[1] + vec2[2] * vec2[2]).sqrt();
    let distance_3: f32 = (vec3[0] * vec3[0] + vec3[1] * vec3[1] + vec3[2] * vec3[2]).sqrt();

    // checks if the player is inside the wall
    // CAUSES THE PLAYER TO SHAKE A BIT
    /* if distance_1 + distance_2 <= distance_3 + 0.9 {
        // vec3 kryss 0,wall height, 0
        let normalen = Vec3::new(
            vec3[1] * 0. - vec3[2] * wall.height,
            vec3[2] * 0. - vec3[0] * 0.,
            vec3[0] * wall.height - vec3[1] * 0.,
        ).normalize_or_zero();

        // ta position plus eller minus normalen 
        player.x += normalen.x / 10.;
        player.z += normalen.z / 10.;
    } */


    if distance_1 + distance_2 <= distance_3 + 1.5 {
        return true
    }

    false
}

pub fn floor_collision(floor: &Mut<'_, Floor>, movement: &mut bevy::prelude::Vec3, player: &mut bevy::prelude::Mut<'_, Player>) {

    let padding = 1.5;


    let position_x = player.x + movement.x;
    let position_z = player.z + movement.z;

    // BEGIN BY CHECKING IF THE PLAYER IS ON THE FLOOR
    //println!("pos: {:?} {:?}", position_x, position_z);

    if is_inside_triangle(floor.a.position.x, floor.a.position.z, floor.b.position.x, floor.b.position.z, floor.c.position.x, floor.c.position.z, position_x, position_z) {
        //println!("inside");

        // CHECK IF PLAYER HITS FLOOR IN y DIRECTION
        // ASUMES THAT ALL y VALUES ARE THE SAME AND THAT THE TRIANGLE IS ALWAYS FLAT
        let position_y = player.y + movement.y - player.height;
        let wall_start_y = floor.a.position.y - padding;
        let wall_end_y = floor.a.position.y + padding;


        //println!("player: {:?} wall: {:?} {:?}", position_y, wall_start_y, wall_end_y);


        if position_y > wall_start_y && position_y < wall_end_y {
            if movement[1] < 0. {
                movement[1] = 0.;
            }
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
    /* println!("first: {:?}", area_1 + area_2 + area_3);
    println!("secon: {:?}", entire_area); */

    return entire_area <= area_1 + area_2 + area_3 + 1. && entire_area >= area_1 + area_2 + area_3 - 1.;
}

/* A utility function to calculate area of triangle formed by (x1, y1), 
   (x2, y2) and (x3, y3) */
fn area(x1: f32,  y1: f32,  x2: f32,  y2: f32,  x3: f32,  y3: f32) -> f32   {
    return ((x1*(y2-y3) + x2*(y3-y1)+ x3*(y1-y2))/2.0).abs();
}

use bevy::prelude::*;
use crate::wall::Wall;
use crate::floor::Floor;
use crate::Player;
/*
    WALL COLLISION

    CHECK IF 

    player.x + movement.x
    player.y + movement.y
    player.z + movement.z

    IS THE SAME VALUE AS WALL POSITION + PADDING

    IF SAME SET movement.z TO 0

    KNOWN BUGs:

    1. player can get stuck in the wall if they walk in towards the edge of a wall, 
    you can still walks alongside the wall but not away from it. 

    TODO: 
    fix bugs
    clean up code 

    // TODO: 
    // walk up stairs (maybe done)

*/

pub fn wall_collision(wall: &Mut<'_, Wall>, movement: &mut bevy::prelude::Vec3, player: &mut bevy::prelude::Mut<'_, Player>) {
    let padding = 1.5;

    // FIND WHICH WAY THE WALL IS FACING ASUMING THAT ALL WALLS ARE FLAT
    
    if wall.start.position.x != wall.end.position.x {
        // WALL IS POINTING IN z DIRECTION
        // CHECK IF PLAYER IS IN SAME (x,y) AS WALL
        if is_inside_rectangle(wall.start.position.x, wall.start.position.y, wall.end.position.x, wall.end.position.y + wall.height, player.x + movement.x, player.y + movement.y, player.height) {
            // CHECK IF PLAYER HITS WALL IN z DIRECTION
            let position_z = player.z + movement.z;
            let wall_start_z = wall.start.position.z - padding;
            let wall_end_z = wall.end.position.z + padding;

            // CHECK IF THE PLAYER IS GOING TO MOVE THROUGH THE WALL
            if position_z > wall_start_z &&  position_z < wall_end_z {
                // walking up stairs
                if player.height / 5. >= wall.height {
                    player.y += wall.height + padding + 1.;
                } else {
                    movement[2] = 0.;
                }
                
            }
        }
    }
    else if wall.start.position.z != wall.end.position.z {
        // WALL IS POINTING IN x DIRECTION
        // CHECK IF PLAYER IS IN SAME (z,y) AS WALL
        if is_inside_rectangle(wall.start.position.z, wall.start.position.y, wall.end.position.z, wall.end.position.y + wall.height, player.z + movement.z, player.y + movement.y, player.height) {
            // CHECK IF PLAYER HITS WALL IN x DIRECTION
            let position_x = player.x + movement.x;
            let wall_start_x = wall.start.position.x - padding;
            let wall_end_x = wall.end.position.x + padding;
            
            // CHECK IF THE PLAYER IS GOING TO MOVE THROUGH THE WALL
            if position_x > wall_start_x &&  position_x < wall_end_x {
                // walking up stairs
                if player.height / 5. >= wall.height {
                    player.y += wall.height + padding + 1.;
                } else {
                    movement[0] = 0.;
                }
                
            }
        }
    }
}

fn infront_of_wall() {
    
}

// function to find if given point
// lies inside a given rectangle or not.
fn is_inside_rectangle(x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32, height: f32) -> bool {
    let mut check_x = false;
    let mut check_y = false;

    if x1 < x2 {
        if x >= x1 && x <= x2 {
            check_x = true
        }
    } else {
        if x <= x1 && x >= x2 {
            check_x = true
        }
    }
    if y1 < y2 {
        if y >= y1 && y <= y2 {
            check_y = true
        }
    } else {
        if y <= y1 && y >= y2 {
            check_y = true
        }
    }

    if check_x && check_y {
        return true;
    }
    return false;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inside_triangle() {
        // Test cases where (x, y) is inside the triangle
        assert!(is_inside_triangle(0.0, 0.0, 4.0, 0.0, 0.0, 4.0, 2.0, 2.0));
        assert!(is_inside_triangle(-1.0, -1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 0.0));
    }

    #[test]
    fn test_outside_triangle() {
        // Test cases where (x, y) is outside the triangle
        assert!(!is_inside_triangle(0.0, 0.0, 4.0, 0.0, 0.0, 4.0, 5.0, 5.0));
        assert!(!is_inside_triangle(-1.0, -1.0, 1.0, -1.0, 0.0, 1.0, 2.0, 2.0));
    }

    #[test]
    fn test_on_boundary_triangle() {
        // Test case where (x, y) is on the boundary of the triangle
        assert!(is_inside_triangle(0.0, 0.0, 4.0, 0.0, 0.0, 4.0, 0.0, 0.0));
    }

    #[test]
    fn test_inside_rectangle() {
        // Test cases where (x, y) is inside the rectangle
        assert!(is_inside_rectangle(0.0, 0.0, 4.0, 4.0, 2.0, 2.0, 4.0));
        assert!(is_inside_rectangle(-1.0, -1.0, 1.0, 1.0, 0.0, 0.0, 2.0));
    }

    #[test]
    fn test_outside_rectangle() {
        // Test cases where (x, y) is outside the rectangle
        assert!(!is_inside_rectangle(0.0, 0.0, 4.0, 4.0, 5.0, 5.0, 1.0));
        assert!(!is_inside_rectangle(-1.0, -1.0, 1.0, 1.0, 2.0, 2.0, 0.5));
    }

    #[test]
    fn test_on_boundary_rectangle() {
        // Test case where (x, y) is on the boundary of the rectangle
        assert!(is_inside_rectangle(0.0, 0.0, 4.0, 4.0, 2.0, 0.0, 4.0));
        assert!(is_inside_rectangle(-1.0, -1.0, 1.0, 1.0, 0.0, 1.0, 2.0));
    }
}
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

fn check_if_wall(player_position: [f32; 3], wall: &Mut<'_, Wall>) -> bool {
    let wall_1: Vec3 = wall.start.position;
    let wall_2: Vec3 = wall.end.position;
    
    let vec1: [f32; 3] = [player_position[0] - wall_1[0], player_position[1] - wall_1[1], player_position[2] - wall_1[2]];
    let vec2: [f32; 3] = [player_position[0] - wall_2[0], player_position[1] - wall_2[1], player_position[2] - wall_2[2]];
    let vec3: [f32; 3] = [wall_1[0] - wall_2[0], wall_1[1] - wall_2[1], wall_1[2] - wall_2[2]];

    let distance_1: f32 = (vec1[0] * vec1[0] + vec1[1] * vec1[1] + vec1[2] * vec1[2]).sqrt();
    let distance_2: f32 = (vec2[0] * vec2[0] + vec2[1] * vec2[1] + vec2[2] * vec2[2]).sqrt();
    let distance_3: f32 = (vec3[0] * vec3[0] + vec3[1] * vec3[1] + vec3[2] * vec3[2]).sqrt();

    if distance_1 + distance_2 <= distance_3 + 1. {
        return true
    }

    false
}

pub fn wall_collision(wall: &Mut<'_, Wall>, movement: &mut bevy::prelude::Vec3, player: &mut bevy::prelude::Mut<'_, Player>) {

    let player_position: [f32; 3] = [player.x + movement.x,
    player.y + movement.y,
    player.z + movement.z,];


    if check_if_wall(player_position, wall) {
        //println!("Wall");
        // calculate normal to determine which way to move
        
        // crossprodukt

        let p1: Vec<f32> = vec![wall.start.position.x , wall.start.position.y, wall.start.position.z ];
        let p2: Vec<f32> = vec![wall.end.position.x, wall.end.position.y, wall.end.position.z];
        let p5: Vec<f32> = vec![wall.start.position.x , wall.start.position.y + wall.height, wall.start.position.z ];

        let v1: Vec<f32> = elementwise_subtraction(&p2, &p1);
        let v2: Vec<f32> = elementwise_subtraction(&p5, &p1);

        let normal: Vec3 = Vec3::new(v1[1]*v2[2] - v1[2]*v2[1],
            v1[2]*v2[0] - v1[0]*v2[2],
            v1[0]*v2[1] - v1[1]*v2[0],
        );

        // movement must be vinkelrätt against normal

        let movement_vec = movement.normalize_or_zero();
        let normal_vec = normal.normalize_or_zero();

        let vec = movement_vec.cross(normal_vec);
        println!("movement: {:?} normal: {:?} vec: {:?}", movement_vec, normal_vec, vec);

        if vec[1] == 0. {
            movement.x = 0.;
            movement.z = 0.;
        } else if vec[1] < 0. {
            movement.x = 0.;
        } else if vec[1] > 0. {
            movement.z = 0.;
        }


        /* let movement_vec = Vec3::new(-normal[2], normal[1], normal[0]);

        //println!("{:?}",normal.dot(movement_vec));

        let movement_vec1: Vec3 = movement_vec.normalize_or_zero();

        // calculate angle between normal and movement_vec
        let angle: f32 = (
            normal.dot(*movement)/
            ((normal[0]*normal[0] + normal[1]*normal[1] + normal[2]*normal[2]).sqrt() * (movement[0]*movement[0] + movement[1]*movement[1] + movement[2]*movement[2]).sqrt())
        ).acos();

        println!("{:?}", angle);
        if angle < 3.14/3. {
            movement.x =  movement_vec1[0];
            movement.z =  movement_vec1[2];
        } else {
            
        } */

        
    }

    /* // https://math.stackexchange.com/questions/1472049/check-if-a-point-is-inside-a-rectangular-shaped-area-3d
    let p1: Vec<f32> = vec![wall.start.position.x + padding, wall.start.position.y, wall.start.position.z + padding];
    let p2 = vec![wall.end.position.x, wall.end.position.y, wall.end.position.z];
    let p4 = vec![wall.start.position.x - padding, wall.start.position.y, wall.start.position.z - padding];
    let p5 = vec![wall.start.position.x + padding, wall.start.position.y + wall.height, wall.start.position.z + padding];
    let pv = vec![player.x + movement.x, player.y + movement.y, player.z + movement.z];
    //println!("before");
    if is_inside_rectangle(&p1, &p2, &p4, &p5, &pv) {
        //println!("hit");
        // kryssprodukt
        let v1 = elementwise_subtraction(&p2, &p1);
        let v2 = elementwise_subtraction(&p5, &p1);

        let normal = vec![
            v1[1]*v2[2] - v1[2]*v2[1],
            v1[2]*v2[0] - v1[0]*v2[2],
            v1[0]*v2[1] - v1[1]*v2[0],
            ];

        let movement_vec = vec![movement.x, movement.y, movement.z];

        // calculate angle between normal and movement_vec
        let angle = (
            dot(&normal, &movement_vec)/
            ((normal[0]*normal[0] + normal[1]*normal[1] + normal[2]*normal[2]).sqrt() * (movement_vec[0]*movement_vec[0] + movement_vec[1]*movement_vec[1] + movement_vec[2]*movement_vec[2]).sqrt())
        ).acos();
        println!("{:?}", angle);
    } */

}

// function to find if given point
// lies inside a given rectangle or not.
fn is_inside_rectangle(p1: &Vec<f32>, p2: &Vec<f32>, p4: &Vec<f32>, p5: &Vec<f32>, pv: &Vec<f32>) -> bool {

    let i= elementwise_subtraction(&p2, &p1);
    let j= elementwise_subtraction(&p4, &p1);
    let k= elementwise_subtraction(&p5, &p1);
    let v= elementwise_subtraction(&pv, &p1);

    if 0. < dot(&v, &i) && dot(&v, &i) < dot(&i, &i) && 0. < dot(&v, &j) && dot(&v, &j) < dot(&j, &j) && 0. < dot(&v, &k) && dot(&v, &k) < dot(&k, &k) {
        return true
    }
    
    return false

}

fn elementwise_subtraction(vec_a: &Vec<f32>, vec_b: &Vec<f32>) -> Vec<f32> {
    vec_a.into_iter().zip(vec_b).map(|(a, b)| a - b).collect()
}

fn dot(p1: &Vec<f32>, p2: &Vec<f32>) -> f32 {
    return p1[0] * p2[0] + p1[1] * p2[1] + p1[2] * p2[2]
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
    fn test_elementwise_subtraction() {
        let vec_a = vec![1.0, 2.0, 3.0];
        let vec_b = vec![0.5, 1.5, 2.5];
        let result = elementwise_subtraction(&vec_a, &vec_b);
        assert_eq!(result, vec![0.5, 0.5, 0.5]);
    }

    #[test]
    fn test_dot_product() {
        let vec_a = vec![1.0, 2.0, 3.0];
        let vec_b = vec![0.5, 1.5, 2.5];
        let result = dot(&vec_a, &vec_b);
        assert_eq!(result, 11.0);
    }

    #[test]
    fn test_is_inside_rectangle() {
        let p1 = vec![0.0, 0.0, 0.0];
        let p2 = vec![2.0, 0.0, 0.0];
        let p4 = vec![0.0, 0.0, 2.0];
        let p5 = vec![0.0, 2.0, 0.0];
        let pv_inside = vec![1.0, 1.0, 1.0];
        let pv_outside = vec![3.0, 3.0, 3.0];
        assert!(is_inside_rectangle(p1.clone(), p2.clone(), p4.clone(), p5.clone(), pv_inside.clone()));
        assert!(!is_inside_rectangle(p1, p2, p4, p5, pv_outside));
    }
}
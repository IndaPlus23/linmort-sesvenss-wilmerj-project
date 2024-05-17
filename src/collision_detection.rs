use crate::floor::Floor;
use crate::wall::Wall;
use crate::Player;
use bevy::prelude::*;
use std::f32::consts::PI;
use bevy::{prelude::*, utils::HashMap};
use crate::sprites::SpriteComponent;

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_detection);
    }
}

fn collision_detection(
    mut commands: Commands,
    mut query: Query<(Entity, &SpriteComponent, &mut Collider)>,
    mut player_query: Query<(Entity, &mut Player, &mut Collider), Without<SpriteComponent>>
) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    // First phase: Detect collisions
    for (entity_a, transform_a, collider_a) in query.iter() {

        // Detect player collision (WORKAROUND since player is not a SpriteComponent and time is running out)
        for (player_entity, mut player, collider_player) in player_query.iter_mut() {
            let distance = transform_a.position.distance(Vec3::new(player.x, player.y, player.z));

            if distance < collider_a.radius + collider_player.radius {
                player.update_health(-10);
                commands.entity(entity_a).despawn_recursive();
            }
        }

        // Detect Sprite on sprite collision
        for (entity_b, transform_b, collider_b) in query.iter() {
            if entity_a != entity_b {
                let distance = transform_a.position
                    .distance(transform_b.position);
                if distance < collider_a.radius + collider_b.radius {
                    colliding_entities
                        .entry(entity_a)
                        .or_insert_with(Vec::new)
                        .push(entity_b);
                }
            }
        }
    }

    // Second phase: Update colliders.
    // TODO: Perform some specific action depending on collision type (sound, projectile etc...)
    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();

        if let Some(collisions) = colliding_entities.get(&entity) {
            collider
                .colliding_entities
                .extend(collisions.iter().copied());
        }
    }
}




/*
    WALL COLLISION

    KNOWN BUGs:

    1. player can get stuck in the wall if they walk in towards the edge of a wall,
    you can still walks alongside the wall but not away from it.
    2. when there are 2 walls in a sharp angle the player can slip through them

    TODO:
    fix bugs
    clean up code

    // TODO:
    // walk up stairs (maybe done)

*/

pub fn wall_collision(
    wall: &Mut<'_, Wall>,
    movement: &mut bevy::prelude::Vec3,
    player: &mut bevy::prelude::Mut<'_, Player>,
) {
    let player_vec: [f32; 3] = [player.x + movement.x, 0., player.z + movement.z];

    if check_if_wall(wall, player_vec, player) {
        // wall vector. if player hits wall it should move alongside the wall aka wall vector
        let wall_vector: Vec3 = Vec3::new(
            wall.start.position.x - wall.end.position.x,
            0.,
            wall.start.position.z - wall.end.position.z,
        )
        .normalize_or_zero();

        // calculate angle between wall_vector and movement
        let angle = movement.angle_between(wall_vector);

        if player.height / 5. >= wall.height {
            // walking up stairs
            player.y += wall.height + 2.;
        } else if PI / 2. - 0.1 < angle && angle < PI / 2. + 0.1 {
            movement.x = 0.;
            movement.z = 0.;
        } else if angle < PI / 2. {
            movement.x = wall_vector.x;
            movement.z = wall_vector.z;
        } else if angle > PI / 2. {
            movement.x = -wall_vector.x;
            movement.z = -wall_vector.z;
        }
    }
}

fn check_if_wall(
    wall: &Mut<'_, Wall>,
    player_vec: [f32; 3],
    player: &mut bevy::prelude::Mut<'_, Player>,
) -> bool {
    // check if player and wall is at same height before starting all calculations
    let wall_start = wall.start.position.y - 1.;
    let wall_end = wall.start.position.y + wall.height + 1.;
    let player_start = player.y - player.height;
    let player_end = player.y;

    if !((wall_start <= player_start && player_start <= wall_end)
        || (wall_start <= player_end && player_end <= wall_end))
    {
        // no wall hit
        return false;
    }

    let wall_1: Vec3 = wall.start.position;
    let wall_2: Vec3 = wall.end.position;

    // vectors from player to the walls 2 corners
    let vec1: Vec3 = Vec3::new(
        player_vec[0] - wall_1[0],
        player_vec[1] - wall_1[1],
        player_vec[2] - wall_1[2],
    );
    let vec2: Vec3 = Vec3::new(
        player_vec[0] - wall_2[0],
        player_vec[1] - wall_2[1],
        player_vec[2] - wall_2[2],
    );
    // wall vector
    let vec3: Vec3 = Vec3::new(
        wall_1[0] - wall_2[0],
        wall_1[1] - wall_2[1],
        wall_1[2] - wall_2[2],
    );

    // calc distance of all 3 vectors
    let distance_1: f32 = (vec1[0] * vec1[0] + vec1[1] * vec1[1] + vec1[2] * vec1[2]).sqrt();
    let distance_2: f32 = (vec2[0] * vec2[0] + vec2[1] * vec2[1] + vec2[2] * vec2[2]).sqrt();
    let distance_3: f32 = (vec3[0] * vec3[0] + vec3[1] * vec3[1] + vec3[2] * vec3[2]).sqrt();

    // checks if the player is inside the wall
    // make sure the player is pushed the right way! (the player is pushed the right way)
    if distance_1 + distance_2 <= distance_3 + 1.4 {

        // vec3 kryss 0,wall height, 0
        let mut normalen = vec3.cross(Vec3::new(0., wall.height, 0.));

        // kolla om vec från wall till player och jämför om normalen är positiv eller negativ multipel

        // ta position plus eller minus normalen
        let check = normalen.dot(vec1);

        normalen = normalen.normalize_or_zero();

        if check > 0. {
            player.x += normalen.x / 5.;
            player.z += normalen.z / 5.;
        } else {
            player.x -= normalen.x / 5.;
            player.z -= normalen.z / 5.;
        }
    }

    // padding must be at least 1.5
    // if the distance of the 2 vectors to the wall from
    // the player is the same as the wall vector, then the
    // player is in the wall
    if distance_1 + distance_2 <= distance_3 + 1.5 {
        return true;
    }

    false
}

pub fn floor_collision(
    floor: &Mut<'_, Floor>,
    movement: &mut bevy::prelude::Vec3,
    player: &mut bevy::prelude::Mut<'_, Player>,
) {
    let padding = 1.;

    let position_x = player.x + movement.x;
    let position_z = player.z + movement.z;

    // BEGIN BY CHECKING IF THE PLAYER IS ON THE FLOOR
    if is_inside_triangle(
        floor.a.position.x,
        floor.a.position.z,
        floor.b.position.x,
        floor.b.position.z,
        floor.c.position.x,
        floor.c.position.z,
        position_x,
        position_z,
    ) {
        // CHECK IF PLAYER HITS FLOOR IN y DIRECTION
        // ASUMES THAT ALL y VALUES ARE THE SAME AND THAT THE TRIANGLE IS ALWAYS FLAT
        let position_y = player.y + movement.y - player.height;
        let y = calc_y(floor.a.position, floor.b.position, floor.c.position, player.x, player.z);
        let wall_start_y = y - padding;
        let wall_end_y = y + padding;


        if wall_start_y < position_y && position_y < wall_end_y {
            if movement[1] != 0. {
                movement[1] = 0.;
            }
        } else if wall_start_y - 1. < position_y && position_y < wall_start_y {
            if movement[1] != 0. {
                movement[1] = 0.;
            }
            player.y = y + player.height + 1.5;
        }
    }
}

/* A function to check whether point P(x, y) lies inside the triangle formed
   by A(x1, y1), B(x2, y2) and C(x3, y3)
   shamelessly stolen from: https://www.geeksforgeeks.org/check-whether-a-given-point-lies-inside-a-triangle-or-not/
*/
fn is_inside_triangle(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    x: f32,
    y: f32,
) -> bool {
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

    return entire_area <= area_1 + area_2 + area_3 + 1.
        && entire_area >= area_1 + area_2 + area_3 - 1.;
}

/* A utility function to calculate area of triangle formed by (x1, y1),
(x2, y2) and (x3, y3) */
fn area(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> f32 {
    return ((x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)) / 2.0).abs();
}
// https://stackoverflow.com/questions/5507762/how-to-find-z-by-arbitrary-x-y-coordinates-within-triangle-if-you-have-triangle

fn calc_y(p1: Vec3, p2: Vec3, p3: Vec3, x: f32, z: f32) -> f32 {
    let det = (p2.z - p3.z) * (p1.x - p3.x) + (p3.x - p2.x) * (p1.z - p3.z);

    let l1 = ((p2.z - p3.z) * (x - p3.x) + (p3.x - p2.x) * (z - p3.z)) / det;
    let l2 = ((p3.z - p1.z) * (x - p3.x) + (p1.x - p3.x) * (z - p3.z)) / det;
    let l3 = 1.0 - l1 - l2;

    return l1 * p1.y + l2 * p2.y + l3 * p3.y;
}
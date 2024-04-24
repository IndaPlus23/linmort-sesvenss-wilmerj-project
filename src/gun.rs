use crate::Player;
use bevy::prelude::*;


pub fn shoot_the_gun(mut query: Query<&mut Player>) {

    // Pitch is how much a player is looking up or down, and yaw is how much they are looking left and right.
    for mut player in query.iter_mut() {
        //println!("yaw: {:?} pitch: {:?}", player.yaw, player.pitch);

        //println!("x: {:?} y: {:?} y: {:?}", player.x, player.y, player.z);

        // send out ray in the direction that the player is facing

        // yaw x, z
        // pitch y

        let mut hit = false;
        let mut ray = [player.x, player.y, player.z];
        let step = 1.;  // how long steps should bullet take when checking for collision
        let range = 15; // range that gun shoots

        for i in 0..range {

            // shot ray in direction player is looking. 
            ray[0] += player.yaw.sin() * step;
            ray[1] += player.pitch.sin() * step;
            ray[2] -= player.yaw.cos() * step;

            // check if bullet hits something
            // maybe use same as wall_collision

            //hit = enemy_detection(ray)
            //hit = wall_detection(ray);
            //hit = flor_detection(ray);

            if hit {
                // do damage?
                break;
            }
        }

        //println!("ray: {:?}", ray)
    }
}
use crate::Player;
use bevy::prelude::*;

#[derive(Default, Component, Clone, PartialEq)]
pub struct Holster {
    pub guns: Vec<Gun>,
    pub in_hand: Option<usize>
}

impl Holster {
    pub fn new() -> Self {
        Self {
            guns: Vec::new(),
            in_hand: None
        }
    }

    pub fn add_gun(mut self, gun: Gun) {
        self.guns.push(gun)
    }

    pub fn del_gun(mut self, gun: Gun) {
        
        for i in 0..self.guns.len() {
            if self.guns[i].gun_name == gun.gun_name {
                self.guns.remove(i);
            }
        }
    }
}
#[derive(Default, Component, Clone, PartialEq)]
pub struct Gun {
    pub gun_name: String,
    pub gun_type: String,
    pub ammo: i32,
    pub max_ammo: i32,
    pub shooting_speed: f32,
    pub reload_speed: f32,
}

impl Gun {
    pub fn new(gun_name: String, gun_type: String, ammo: i32, max_ammo: i32, shooting_speed: f32, reload_speed: f32) -> Self {
        Self {
            gun_name,
            gun_type,
            ammo,
            max_ammo,
            shooting_speed,
            reload_speed,
        }
    }

    pub fn basic_new(gun_type: String, max_ammo: i32) -> Self {
        Self {
            gun_name: gun_type.clone(),
            gun_type,
            ammo:max_ammo,
            max_ammo,
            shooting_speed:1.,
            reload_speed:1.,
        }
    }
}

pub fn shoot_the_gun(mut query: Query<&mut Player>) {

    // Pitch is how much a player is looking up or down, and yaw is how much they are looking left and right.
    for player in query.iter_mut() {

        // todo:
            // health bar
            // gun hit detection


        
        // send out ray in the direction that the player is facing
        // yaw x, z
        // pitch y

        let mut hit = false;
        let mut ray = [player.x, player.y, player.z];
        let step = 0.25;  // how long steps should bullet take when checking for collision
        let range = 100; // range is how many steps the bullet should take


        // range is how long the gun shoots, add to range to shoot longer
        for _i in 0..range {

            // shot ray in direction player is looking. 
            ray[0] += player.yaw.sin() * step;
            ray[1] += player.pitch.sin() * step;
            ray[2] -= player.yaw.cos() * step;

            // check if bullet hits something
            // HIT DETECTION

            if hit {
                // do damage?
                break;
            }
        }

        //println!("ray: {:?}", ray)
    }
}
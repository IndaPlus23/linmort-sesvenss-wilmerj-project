use bevy::prelude::*;
use nalgebra::{Rotation3, Unit, Vector3};
use std::f32::consts::PI;

pub const PLAYER_PROJECTILE_SPEED: f32 = 1000.;
pub const PLAYER_HIT_RADIUS: f32 = 10.;

#[derive(Default, Component, Clone, PartialEq)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub height: f32,
    pub noclip: bool,
    pub gravity: f32,
    pub health: i32,
    pub ammo: i32,
}

impl Player {
    pub fn new(x: f32, y: f32, z: f32, yaw: f32, pitch: f32) -> Self {
        Self {
            x,
            y,
            z,
            yaw,
            pitch,
            height: 8.,
            noclip: true,
            gravity: 0.,
            health: 100,
            ammo: 7,
        }
    }

    pub fn update_health(&mut self, amount: i32) {
        self.health += amount;

        if self.health <= 0 {
            // TODO: Change game state to dead
        }
    }

    // TODO: COPY FROM SKYBOX, REMOVE WHEN MERGING
    pub fn forward_vector(&self) -> Vec3 {
        let mut vector = Vector3::new(0., 0., -1.);

        //create rotation matrices from yaw and pitch
        let yaw_rotation = Rotation3::from_euler_angles(0., 2. * PI - self.yaw, 0.);
        vector = yaw_rotation * vector;

        let axis = Unit::new_normalize(vector.cross(&Vector3::y()));
        let pitch_rotation = Rotation3::from_axis_angle(&axis, self.pitch);

        vector = pitch_rotation * vector;

        Vec3::new(vector.x, vector.y, vector.z)
    }
}

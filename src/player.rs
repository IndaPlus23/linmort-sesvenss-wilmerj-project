use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, z: f32, yaw: f32, pitch: f32) -> Self {
        Self {
            x,
            y,
            z,
            yaw,
            pitch,
        }
    }
}
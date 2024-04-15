use bevy::prelude::*;
use crate::player::Player;

#[derive(Component)]
pub struct Sprite {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Sprite {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
}
use bevy::prelude::*;
use crate::gun::Holster;

#[derive(Default, Component, Clone, PartialEq)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub holster: Holster,
    pub height: f32,
    pub noclip: bool,
    pub gravity: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, z: f32, yaw: f32, pitch: f32) -> Self {

        Self {
            x,
            y,
            z,
            yaw,
            pitch,
            holster: Holster::new(),
            height: 5.,
            noclip: true,
            gravity: 0.,
        }
    }
}

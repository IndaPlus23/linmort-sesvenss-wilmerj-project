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
}

impl Player {
    pub fn new(x: f32, y: f32, z: f32, yaw: f32, pitch: f32, holster: Holster) -> Self {

        Self {
            x,
            y,
            z,
            yaw,
            pitch,
            holster
        }
    }
}

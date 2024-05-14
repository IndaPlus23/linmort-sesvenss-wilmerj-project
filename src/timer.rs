use bevy::prelude::{Component, Timer, TimerMode};

/// Used to time events
#[derive(Component)]
pub struct ShootingTimer {
    pub timer: Timer,
}
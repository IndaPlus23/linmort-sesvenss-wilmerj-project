use bevy::prelude::{Component, Deref, DerefMut, Timer, TimerMode};

/// Used to time events
#[derive(Component)]
pub struct ShootingTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct WalkTimer {
    pub timer: Timer,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);



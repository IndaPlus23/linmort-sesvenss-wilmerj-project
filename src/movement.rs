use bevy::prelude::*;
use crate::enemy::ActionState;
use crate::sprites::SpriteComponent;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self { Self { value }}
}

#[derive(Component)]
pub struct Acceleration {
    pub value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self { Self { value }}
}

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub sprite: SpriteBundle,
    pub state: ActionState,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_position, update_velocity));
    }
}

/// update_velocity updates the current velocity of a moving object.
/// !IMPORTANT Notice that time is used to standardize movement instead of relying on computer performance.
fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds();
    }
}

/// update_velocity updates the current velocity of a moving object.
/// !IMPORTANT Notice that time is used to standardize movement instead of relying on computer performance.
fn update_position(mut query: Query<(&Velocity, &mut SpriteComponent)>, time: Res<Time>) {
    // TODO: Query enemycomponent instead, transform.translation does not represent the position of the sprite. Position does that.
    for (velocity, mut sprite) in query.iter_mut() {
        sprite.position+= velocity.value  * time.delta_seconds();
    }
}
use bevy::prelude::*;
use crate::collision_detection::Collider;
use crate::movement::Velocity;
use bevy::ecs::component::Component;
use crate::player::Player;
use crate::sound::Sound;

#[derive(Component, Clone, Copy, Debug)]
enum EnemyState {
    Dormant,
    Attacking,
    Dying,
    Dead,
}

#[derive(Component)]
pub struct EnemyComponent {
    pub position: Vec3,
    pub height: f32,
}

// Enemy stats are stored in JSON format.
#[derive(Component, Clone, Copy, Debug)]
pub struct Enemy {
    pub(crate) position: Vec3,
    state: EnemyState,
    reaction_speed: usize,
    speed: usize,
    hp: usize,
    attack: usize,
    range: usize,
    respawn_time: Option<usize>, // If true, usize
    projectile_speed: usize,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
            app.add_systems(Update, act);
    }
}

impl Enemy {
    /// Creates a new enemy with an associated sprite and
    pub fn new(
        reaction_speed: usize,
        speed: usize,
        hp: usize,
        attack: usize,
        range: usize,
        respawn_time: Option<usize>,
        projectile_speed: usize,
    ) -> Self {
        Enemy {
            position: Vec3::ZERO,
            state: EnemyState::Dormant,
            reaction_speed,
            speed,
            hp,
            attack,
            range,
            respawn_time,
            projectile_speed,
        }
    }

    pub fn update_position(mut self, pos: Vec3) {
        self.position = pos;
    }

    // TODO: transform and screen are copies from sprite.rs which is a copy from vertex.rs
    pub fn transform(
        position: Vec3,
        player: &Player
    ) -> Vec3 {

        // This code comes from transform_vertice
        let mut x = position.x;
        let mut y = position.y;
        let mut z = position.z;

        let cos = player.yaw.cos();
        let sin = player.yaw.sin();

        x -= player.x;
        y -= player.y;
        z -= player.z;

        let new_x = x * cos + z * sin;
        let new_z = z * cos - x * sin;
        let new_y = y + (player.pitch * new_z);

        Vec3::new(new_x, new_y, new_z)
    }

    pub fn screen(position: Vec3) -> Vec2 {
        let world_x = position.x;
        let world_y = position.y;
        let world_z = position.z;

        let screen_x = world_x * 1500. / world_z;
        let screen_y = world_y * 1500. / world_z;

        Vec2::new(-screen_x, -screen_y)
    }
}

/// The "AI" of enemies. Loops over all enemies in
fn act(mut commands: Commands, mut query: Query<(&mut Transform, &mut Velocity), With<Enemy>>) {
    for (mut transform, mut velocity) in query.iter() {
        // TODO: Loop over available enemies, and check their state. Take different actions depending on state.
        // TODO: Detect sounds
        // TODO: Follow walk in random directions based on walls around the enemy
    }
}

fn handle_enemy_sound_collisions(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<Sound>>
) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            // TODO: Deal with collisions
        }
    }
}

//TODO: Delete enemy. Removes from act loop and from game once killed
use bevy::prelude::*;
use crate::collision_detection::Collider;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use bevy::ecs::component::Component;
use bevy::log::tracing_subscriber::fmt::time;
use crate::asset_loader::SceneAssets;
use crate::player::Player;
use crate::sound::Sound;
use bevy::time::Timer;
use crate::sprites::SpriteComponent;
use crate::timer::ShootingTimer;
use crate::utility::normalize;

const MISSILE_SPEED: f32 = 100.;

#[derive(Clone, Copy, Debug, Component)]
pub enum ActionState {
    Dormant,
    Attacking,
    Dead,
}

#[derive(Component)]
pub struct EnemyState {
    pub(crate) state: ActionState
}

// Enemy stats are stored in JSON format.
#[derive(Component, Clone, Copy, Debug)]
pub struct Enemy {
    pub(crate) position: Vec3,
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
        app.add_systems(Update, (act, handle_projectile_collisions));
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

#[derive(Component)]
struct DelayedAction {
    timer: Timer,
}
/// The "AI" of enemies. Loops over all enemies in
/// Update velocity to change direction of movement
fn act(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    time: Res<Time>,
    mut player_query: Query<&Player>,
    mut enemy_query: Query<(
        &Velocity,
        &Transform,
        &mut EnemyState,
        &mut SpriteComponent,
        &mut ShootingTimer
    )>,
) {
    for (
        velocity,
        transform,
        state,
        enemy,
        mut timer
    ) in enemy_query.iter_mut() {
        match state.state {
            ActionState::Dormant => {
                // Shoot if enemy state is attacking
                timer.timer.tick(time.delta());

                let player = player_query.single();

                // TODO: Calculate actual direction
                let direction = normalize(Vec3::new(player.x, player.y, player.z));

                if timer.timer.finished() {
                    create_projectile(&mut commands, scene_assets.projectile.clone(), enemy.position, direction);
                }
            }
            _ => {}
        }
    }
}


// TODO: Detect collisions correctly
fn handle_projectile_collisions(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<ProjectileComponent>>
) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {

            println!("Collision detected");

            // TODO: Projectile collides with enemy once spawning
            if query.get(collided_entity).is_ok() {
                continue;
            }

            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Component)]
struct ProjectileComponent;

fn create_projectile(
    commands: &mut Commands,
    sprite: Handle<Image>,
    position: Vec3,
    direction: Vec3,
) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::new(direction.x, direction.y, direction.z) * MISSILE_SPEED),
            acceleration: Acceleration::new(Vec3::ZERO),
            collider: Collider::new(50.),
            sprite: SpriteBundle {
                texture: sprite,
                transform: Transform::from_translation(position),
                ..default()
            },
        },
        SpriteComponent {
            position,
            height: 10.,
        },
        ProjectileComponent,
    ));
}
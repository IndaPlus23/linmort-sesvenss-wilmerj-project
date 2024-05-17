use crate::asset_loader::SceneAssets;
use crate::collision_detection::Collider;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use crate::player::Player;
use crate::sprites::SpriteComponent;
use crate::timer::{ShootingTimer, WalkTimer};
use crate::utility::normalize;
use crate::wall::Wall;
use bevy::ecs::component::Component;
use bevy::prelude::*;
use bevy::time::Timer;
use rand::Rng;
use std::f32::consts::PI;
use std::time::Duration;

const MISSILE_SPEED: f32 = 150.;
const ENEMY_MOVEMENT_SPEED: f32 = 20.;

// Used to spawn projectiles from outside of hit radius of enemy
pub const ENEMY_PROJECTILE_RADIUS: f32 = 11.;

#[derive(Clone, Copy, Debug, Component)]
pub enum ActionState {
    Dormant,
    Attacking,
    Dying,
    Dead,
}

#[derive(Component)]
pub struct EnemyState {
    pub(crate) state: ActionState,
}

struct Movement {
    direction: Vec2,
    duration: Duration,
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
    pub fn transform(position: Vec3, player: &Player) -> Vec3 {
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
        &mut Velocity,
        &Transform,
        &mut EnemyState,
        &mut SpriteComponent,
        &mut ShootingTimer,
        &mut WalkTimer,
    )>,
) {
    for (
        mut velocity,
        transform,
        mut state,
        enemy,
        mut shooting_timer,
        mut walk_timer
    ) in enemy_query.iter_mut() {
        // Combat actions
        match state.state {
            ActionState::Dormant => {

                // Random walking
                walk_timer.timer.tick(time.delta());

                if walk_timer.timer.finished() {
                    let movement = generate_random_movement();

                    velocity.value = Vec3::new(movement.direction.x, 0., movement.direction.y) * ENEMY_MOVEMENT_SPEED;
                    walk_timer.timer = Timer::new(movement.duration, TimerMode::Once);
                }

                // Shoot if enemy state is attacking
                shooting_timer.timer.tick(time.delta());

                let player = player_query.single();

                let direction = normalize(Vec3::new(player.x, player.y, player.z) - enemy.position);

                if shooting_timer.timer.finished() {
                    state.state = ActionState::Attacking;
                    create_projectile(
                        &mut commands,
                        scene_assets.projectile.clone(),
                        enemy.position,
                        direction,
                        ENEMY_PROJECTILE_RADIUS,
                        MISSILE_SPEED,
                    );
                }
            }
            ActionState::Dying => {
                velocity.value = Vec3::ZERO;
            }
            ActionState::Dead => {
                velocity.value = Vec3::ZERO;
            }
            _ => {}
        }
    }
}

fn handle_projectile_collisions(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<ProjectileComponent>>,
    projectile_query: Query<(Entity, &Collider), With<ProjectileComponent>>,
    mut enemy_query: Query<(Entity, &mut EnemyState)>
) {
    // Iterate over all projectiles
    for (projectile_entity, collider) in projectile_query.iter() {
        // Check each colliding entity
        for &collided_entity in collider.colliding_entities.iter() {
            // TODO: Projectile collides with enemy once spawning
            if query.get(collided_entity).is_ok() {
                continue;
            }
            // Check if the collided entity is an enemy and if so, modify its state
            if let Ok((_, mut enemy_state)) = enemy_query.get_mut(collided_entity) {
                enemy_state.state = ActionState::Dead;
                commands.entity(projectile_entity).despawn();
            }
        }
    }
}

#[derive(Component)]
struct ProjectileComponent;

pub fn create_projectile(
    commands: &mut Commands,
    sprite: Handle<Image>,
    position: Vec3,
    direction: Vec3,
    radius: f32,
    projectile_speed: f32,
) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(direction * projectile_speed),
            acceleration: Acceleration::new(Vec3::ZERO),
            sprite: SpriteBundle {
                texture: sprite,
                transform: Transform::from_translation(Vec3::new(100000., 1000000., 1000000.)),
                ..default()
            },
        },
        Collider::new(5.),
        SpriteComponent {
            position: (position + direction * radius),
            health: 10.,
        },
        ProjectileComponent,
    ));
}

fn generate_random_movement() -> Movement {
    let mut rng = rand::thread_rng();

    // Generate a random angle in radians
    let angle: f32 = rng.gen_range(0.0..2.0 * PI);

    // Convert the angle to a 2D vector
    let direction = Vec2::new(angle.cos(), angle.sin());

    // Generate a random duration between 0.5 and 3.0 seconds
    let duration: Duration = Duration::from_secs(rng.gen_range(0.5..5.0) as u64);

    Movement { direction, duration }
}

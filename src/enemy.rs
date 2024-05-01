use bevy::asset::{UntypedAssetId, VisitAssetDependencies};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::asset_loader::SceneAssets;
use crate::collision_detection::Collider;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use crate::sound::Sound;

#[derive(Clone, Debug)]
enum EnemyState {
    Dormant,
    Attacking,
    Dying,
    Dead,
}

// TODO: Use the Transform component instead of a custom position
// Enemy stats are stored in JSON format.
#[derive(Component, Clone, Debug, Default)]
pub struct Enemy {
    id: usize,
    position: Vec3,
    state: EnemyState,
    reaction_speed: usize,
    speed: usize,
    hp: usize,
    attack: usize,
    range: usize,
    respawn_time: Option<usize>, // If true, usize
    projectile_speed: usize,
    texture: String, // Used to query texture in SceneBundle
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
            app.add_systems(Update, act);
    }
}

impl Enemy {
    /// Creates a new enemy with an associated sprite and
    pub fn new(id: usize, position: Vec3, enemy_type: String) -> Self {
        Enemy {
            id,
            position,
            state: EnemyState::Dormant,
            reaction_speed: 0,
            speed: 0,
            hp: 0,
            attack: 0,
            range: 0,
            respawn_time: None,
            projectile_speed: 0,
            texture: enemy_type,
        }
    }

    /// Populates map with enemies
    pub fn spawn_enemy(commands: &mut Commands, scene_assets: &mut Res<SceneAssets>, enemy: &Enemy) {

        let sprite = match enemy.texture.as_str() {
            "enemy_a" => { scene_assets.enemy.clone()},
            _ => panic!("Couldn't recognize enemy type: {} from file.", enemy.texture)
        };

        // TODO: Bundle enemy entity with spawned entity. Currently, there is no indication of enemy type
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(Vec3::ZERO),
                acceleration: Acceleration::new(Vec3::ZERO),
                model: SceneBundle {
                    scene: sprite,
                    transform: Transform::from_translation(enemy.position),
                    ..default()
                },
            }
        ));
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
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::asset_loader::SceneAssets;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};

enum EnemyState {
    Walking,
    Attacking,
    Dying,
    Dead,
}

// Enemy stats are stored in JSON format.
#[derive(Serialize, Deserialize)]
struct Enemy {
    state: EnemyState,
    reaction_speed: usize,
    speed: usize,
    hp: usize,
    attack: usize,
    range: usize,
    respawn_time: Option<usize>, // If true, usize
    projectile_speed: usize,
    sprite_sheet: SpriteSheetBundle,
}
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
            app.add_systems(PostStartup, spawn_enemies)
                .add_systems(Update, act);
    }
}

/// Populates map with enemies
fn spawn_enemies(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    // TODO: Loop over enemies in map and determine position and type of enemy
    for enemy in todo!() {
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(Vec3::ZERO),
                acceleration: Acceleration::new(Vec3::ZERO),
                model: SceneBundle {
                    scene: scene_assets.enemy.clone(),
                    transform: Transform::from_translation(enemy.position),
                    ..default()
                },
            }
        ));
    }
}

/// The "AI" of enemies.
fn act(mut commands: Commands, mut query: Query<(&mut Transform, &mut Velocity), With<Enemy>>) {
    for (mut transform, mut velocity) in query.iter() {
        // TODO: Loop over available enemies, and check their state. Take different actions depending on state.
        // TODO: Detect sounds
        // TODO: Follow walk in random directions based on walls around the enemy
    }
}
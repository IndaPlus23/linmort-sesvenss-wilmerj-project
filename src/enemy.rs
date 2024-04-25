use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::asset_loader::SceneAssets;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};

#[derive(Clone)]
enum EnemyState {
    Dormant,
    Attacking,
    Dying,
    Dead,
}

// Enemy stats are stored in JSON format.
#[derive(Component, Clone, Serialize, Deserialize)]
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
    sprite_sheet: Handle<Scene>,
}
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
            app.add_systems(Update, act);
    }
}

impl Enemy {

    /// Creates a new enemy with an associated sprite and
    pub fn new(id: usize, position: Vec3, enemy_type: usize) -> Self {

        let mut sprite: Sprite;

        // TODO: Load enemy data from file depending on the enemy_type
        match enemy_type {
            1 => {todo!()},
            _ => error!("Couldn't recognize enemy type: {} from file.", enemy_type)
        }

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
            sprite_sheet: Default::default(),
        }
    }

    /// Populates map with enemies
    pub fn spawn_enemy(commands: &mut Commands, scene_assets: &mut Res<SceneAssets>, enemy: Enemy) {
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



/// The "AI" of enemies. Loops over all enemies in
fn act(mut commands: Commands, mut query: Query<(&mut Transform, &mut Velocity), With<Enemy>>) {
    for (mut transform, mut velocity) in query.iter() {
        // TODO: Loop over available enemies, and check their state. Take different actions depending on state.
        // TODO: Detect sounds
        // TODO: Follow walk in random directions based on walls around the enemy
    }
}

//TODO: Delete enemy. Removes from act loop and from game once killed
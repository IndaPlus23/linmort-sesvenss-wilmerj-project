use bevy::asset::{UntypedAssetId, VisitAssetDependencies};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::asset_loader::SceneAssets;
use crate::collision_detection::Collider;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use crate::player::Player;
use crate::sound::Sound;
use crate::vertice::Vertice;

#[derive(Clone, Debug)]
enum EnemyState {
    Dormant,
    Attacking,
    Dying,
    Dead,
}

// TODO: Use the Transform component instead of a custom position
// Enemy stats are stored in JSON format.
#[derive(Component)]
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
    pub(crate) texture: Handle<Scene>, // Used to query texture in SceneBundle
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
            app.add_systems(Update, act);
    }
}

impl Enemy {
    /// Creates a new enemy with an associated sprite and
    pub fn new(id: usize,
               reaction_speed: usize,
               speed: usize,
               hp: usize,
               attack: usize,
               range: usize,
               respawn_time: Option<usize>,
               projectile_speed: usize,
               enemy_type: String,
               scene_assets: &Res<SceneAssets>,
    ) -> Self {

        // TODO: find enemy sprite
        let sprite_handle = match enemy_type.as_str() {
            "enemy_a" => { scene_assets.enemy.clone()},
            _ => panic!("Couldn't recognize enemy type: {} from file.", enemy_type)
        };

        Enemy {
            id,
            position: Vec3::ZERO,
            state: EnemyState::Dormant,
            reaction_speed,
            speed,
            hp,
            attack,
            range,
            respawn_time,
            projectile_speed,
            texture: sprite_handle,
        }
    }

    /// Populates map with enemies<
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

    pub fn transform(
        &mut self,
        player: &Player,
    ) -> Vec2 {
        let mut x = self.position.x;
        let mut y = self.position.y;
        let mut z = self.position.z;

        let cos = player.yaw.cos();
        let sin = player.yaw.sin();

        x -= player.x;
        y -= player.y;
        z -= player.z;

        let new_x = x * cos + z * sin;
        let new_z = z * cos - x * sin;
        let new_y = y + (player.pitch * new_z);

        let position = Vec3::new(new_x, new_y, new_z);

        // Enemy is behind player, should not render.
        if position.z > 0. {
            return Vec2::ZERO
        }

        // ------------------------------------ Secured

        // TODO: Might have to deal with clipping

        return position.screen();
    }

    /// Converts vertice coordinates to 2d screen coordinates
    /// NOTE: COPY FROM vertice.rs
    pub fn screen(&self) -> Vec2 {
        let world_x = self.position.x;
        let world_y = self.position.y;
        let world_z = self.position.z;

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
use bevy::prelude::*;
use crate::asset_loader::SceneAssets;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use serde::{Deserialize, Serialize};

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

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

pub struct SpritePlugin;

// TODO: Implement sprite spawning at various part of the game
impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_sprite);
    }
}

// TODO: Don't render a sprite which won't be visible on screen. Instead, use some other mechanic to track the sprite while "walking" off-screen.
/// spawn_sprites uses SceneAssets to clone handles for different assets. This means that the
/// asset itself won't be loaded from file each time an asset is spawned, but rather the handle
/// for that asset is cloned and used.
fn spawn_sprite(mut commands: Commands, scene_assets: Res<SceneAssets>, position: Vec3) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity { value: Vec3::ZERO },
            acceleration: Acceleration::new(Vec3::ZERO),
            model: SceneBundle {
                scene: scene_assets.enemy.clone(),
                transform: Transform::from_translation(position),
                ..default()
            },
        }
    ));
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}
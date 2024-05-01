use bevy::prelude::*;
use crate::asset_loader::SceneAssets;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use serde::{Deserialize, Serialize};
use std::f64;
use crate::utility;

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
        todo!()
    }
}

pub struct Sprite {
    position: Vec3,
    scale: usize,
}

impl Sprite {
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

    // TODO: Implement clip

    // TODO: Make function work on sprites
    fn screen(&self) -> Vec2 {
        let world_x = self.position.x;
        let world_y = self.position.y;
        let world_z = self.position.z;

        let screen_x = world_x * 1500. / world_z;
        let screen_y = world_y * 1500. / world_z;

        Vec2::new(-screen_x, -screen_y)
    }
}
use bevy::prelude::*;
use crate::asset_loader::SceneAssets;
use crate::enemy::Enemy;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use crate::player::Player;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

pub struct SpritePlugin;

// TODO: Implement sprite spawning at various part of the game
// impl Plugin for SpritePlugin {
//     fn build(&self, app: &mut App) {
//         todo!()
//     }
// }

pub struct Sprite {
    position: Vec3,
    height: usize,
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

    // Transforms the sprite's position based on the player's position and orientation
    pub fn transform(
        &self,
        player: &Player
    ) -> Option<(Vec2, Vec2)> {
        let view_position = self.position - player.position;
        let view_position = player.orientation * view_position; // Assuming orientation is a Quat or similar

        // Perform simple frustum culling
        if view_position.z <= 0.0 {
            // The sprite is behind the player
            None
        } else {
            // Convert 3D position to 2D screen coordinates
            let screen_x = view_position.x / view_position.z;
            let screen_y = view_position.y / view_position.z;
            let screen_position = Vec2::new(screen_x, screen_y);

            // Calculate the size on the screen based on the distance
            let scale = 1.0 / view_position.z; // Simple perspective scaling
            let screen_size = self.height * scale;

            Some((screen_position, screen_size))
        }
    }

    /// Apply transformation based on player rotation and position.
    pub fn transform_sprite(&self, player: &Player) -> Enemy {

        // TODO: Query enemies

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

        // TODO: Fix return/mod
    }

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
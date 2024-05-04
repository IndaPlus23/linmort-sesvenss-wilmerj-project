use bevy::prelude::*;
use crate::asset_loader::SceneAssets;
use crate::enemy::Enemy;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use crate::player::Player;
use crate::vertice::Vertice;

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
    ) -> Vec2 {

        // This code comes from transform_vertice
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

        // This code comes from transform

        // Do not render if position is behind player
        return if self.position.z > 0. {
            Vec2::ZERO
        } else {
            // TODO: Might have to do with clipping if end/start is behind player
            // TODO: Might have to deal with scaling issues. Something like self.scale = screen/z

            self.screen()
        }
    }

    fn screen(&self) -> Vec2 {
        let world_x = self.position.x;
        let world_y = self.position.y;
        let world_z = self.position.z;

        let screen_x = world_x * 1500. / world_z;
        let screen_y = world_y * 1500. / world_z;

        Vec2::new(-screen_x, -screen_y)
    }
}
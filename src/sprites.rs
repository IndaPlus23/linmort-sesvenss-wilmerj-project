use bevy::prelude::*;
use crate::asset_loader::SceneAssets;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};

// TODO: Remove, this is temporary
const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., -20.0);

pub struct SpritePlugin;

// TODO: Implement sprite spawning at various part of the game
impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_sprite);
    }
}

// TODO: Make more modular by not spawning a set sprite on a set location
// TODO: Don't render a sprite which won't be visible on screen. Instead, use some other mechanic to track the sprite while "walking" off-screen.
/// spawn_sprites uses SceneAssets to clone handles for different assets. This means that the
/// asset itself won't be loaded from file each time an asset is spawned, but rather the handle
/// for that asset is cloned and used.
fn spawn_sprite(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity { value: Vec3::ZERO },
            acceleration: Acceleration::new(Vec3::ZERO),
            model: SceneBundle {
                scene: scene_assets.enemy.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        }
    ));
}
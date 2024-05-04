use bevy::prelude::*;
use std::path::PathBuf;
use serde_json;
use std::fs;
use serde::{Deserialize, Serialize};
use crate::enemy::Enemy;
use std::path::Path;
use std::collections::HashMap;

/// SceneAssets stores handles for assets used in the scene.
#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub enemy: Handle<Image>,
    pub enemy_types: HashMap<String, Enemy>,
    pub textures: Vec<Handle<Image>>,
    pub texture_paths: Vec<String>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

/// Loads assets from asset folder and populates AssetScene, making them available
/// for usage without having multiple handles reference various copies of the same asset.
pub fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    // TODO: Replace with a more permanent solution
    let texture_paths = vec![String::from("grass_front.png"), String::from("SFLR6_4.png")];

    *scene_assets = SceneAssets {
        enemy: asset_server.load(Path::new("sprites/enemy.png")),
        enemy_types: load_enemy_types(),
        textures: load_textures_from_folder(texture_paths.clone(), asset_server),
        texture_paths: texture_paths,
    }
}

/// Loads folder of textures and upgrades into handle of image
fn load_sprite_sheet(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    tile_size: Vec2,
    columns: usize,
    rows: usize,
    padding: Option<Vec2>,
    offset: Option<Vec2>,
    file_name: String,
) -> Handle<TextureAtlasLayout> {
    let texture: Handle<Image> = asset_server.load("spritesheet.png");
    let layout = TextureAtlasLayout::from_grid(tile_size, columns, rows, None, None);
    texture_atlas_layouts.add(layout)
}

/// Loads folder of textures and upgrades into handle of image
fn load_textures_from_folder(
    texture_paths: Vec<String>,
    asset_server: Res<AssetServer>,
) -> Vec<Handle<Image>> {
    let mut image_handles: Vec<Handle<Image>> = Vec::new();

    for texture in texture_paths.iter() {
        let mut path = PathBuf::from("textures/");
        path.push(texture);
        // Unchecked loading
        let texture: Handle<Image> = asset_server.load(path);
        image_handles.push(texture);
    }

    return image_handles;
}

#[derive(Serialize, Deserialize)]
struct EnemyJSON {
    state: String,
    reaction_speed: usize,
    speed: usize,
    hp: usize,
    attack: usize,
    range: usize,
    respawn_time: Option<usize>, // Using Option to handle boolean/None scenario
    projectile_speed: usize,
    texture_name: Option<String>, // Optional because not all enemies might have it
    sprite_sheet: Option<String>, // Optional for different key names in JSON
    columns: usize,
    rows: usize,
}

fn load_enemy_types() -> HashMap<String, Enemy> {
    let data = fs::read_to_string("./assets/enemies/enemies.json").expect("Unable to read file");
    let enemy_datas: HashMap<String, EnemyJSON> = serde_json::from_str(&data).expect("JSON was incorrectly formatted");

    let mut enemies: HashMap<String, Enemy> = Default::default();

    for (id, enemy) in enemy_datas {
        enemies.insert(id, Enemy::new(
            enemy.reaction_speed,
            enemy.speed,
            enemy.hp,
            enemy.attack,
            enemy.range,
            enemy.respawn_time,
            enemy.projectile_speed,
        ));
    }

    enemies
}
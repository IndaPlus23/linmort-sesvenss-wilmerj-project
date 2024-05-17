use bevy::prelude::*;
use std::{
    fs,
    path::{Path, PathBuf},
    collections::HashMap,
};
use serde_json;
use serde::{Deserialize, Serialize};
use crate::enemy::Enemy;

/// SceneAssets stores handles for assets used in the scene.
#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub enemy: Handle<Image>,
    pub hud: Vec<Handle<Image>>,
    pub cubemaps: Vec<Handle<Image>>,
    pub enemy_types: HashMap<String, Enemy>,
    pub textures: Vec<Handle<Image>>,
    pub texture_paths: Vec<String>,
    pub enemy_a_spritelayout: Handle<TextureAtlasLayout>,
    pub enemy_a_spritesheet: Handle<Image>,
    pub projectile: Handle<Image>,
    pub shotgun_sprite: Handle<Image>,
    pub shotgun_spritelayout: Handle<TextureAtlasLayout>,
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
pub fn load_assets(
    mut scene_assets: ResMut<SceneAssets>,
    mut asset_server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>
) {
    let hud_paths = texture_paths("assets\\textures\\hud\\");
    let cubemap_paths = texture_paths("assets\\textures\\cubemap\\");
    let texture_paths = texture_paths("assets\\textures\\flats\\");

    *scene_assets = SceneAssets {
        hud: load_textures_from_folder(
            hud_paths.clone(),
            &mut asset_server,
            "textures/hud/".to_string(),
        ),
        cubemaps: load_textures_from_folder(
            cubemap_paths.clone(),
            &mut asset_server,
            "textures/cubemap/".to_string(),
        ),
        textures: load_textures_from_folder(
            texture_paths.clone(),
            &mut asset_server,
            "textures/flats/".to_string(),
        ),
        enemy: asset_server.load(Path::new("sprites/enemy.png")),
        projectile: asset_server.load(Path::new("sprites/projectile.png")),
        enemy_a_spritesheet: asset_server.load(Path::new("spritesheets/spritesheet.png")),
        enemy_a_spritelayout: load_sprite_sheet_layout(
            &mut asset_server,
            &mut texture_atlas_layout,
            Vec2::new(116.8, 114.3),
            5,
            3,
            None,
            None,
            "spritesheet.png",
        ),
        enemy_types: load_enemy_types(),
        texture_paths,
        shotgun_sprite: asset_server.load(Path::new("spritesheets/shotgun.png")),
        shotgun_spritelayout: load_sprite_sheet_layout(
            &mut asset_server,
            &mut texture_atlas_layout,
            Vec2::new(454., 256.),
            7,
            1,
            None,
            None,
            "spritesheet.png",
        ),
    }
}

/// Loads folder of textures and upgrades into handle of image
fn load_sprite_sheet_layout(
    asset_server: &mut Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    tile_size: Vec2,
    columns: usize,
    rows: usize,
    padding: Option<Vec2>,
    offset: Option<Vec2>,
    file_name: &str,
) -> Handle<TextureAtlasLayout> {
    // let base_path = Path::new("spritesheets");
    // let full_path = base_path.join(file_name);
    // let texture: Handle<Image> = asset_server.load(full_path);
    let layout = TextureAtlasLayout::from_grid(tile_size, columns, rows, None, None);
    texture_atlas_layouts.add(layout)
}

/// Loads folder of textures and upgrades into handle of image
fn load_textures_from_folder(
    texture_paths: Vec<String>,
    asset_server: &mut Res<AssetServer>,
    path: String,
) -> Vec<Handle<Image>> {
    let mut image_handles: Vec<Handle<Image>> = Vec::new();

    for texture in texture_paths.iter() {
        let mut path_buf = PathBuf::from(path.clone());
        path_buf.push(texture);
        // Unchecked loading
        let texture: Handle<Image> = asset_server.load(path_buf.clone());
        image_handles.push(texture);
    }

    return image_handles;
}

fn texture_paths(folder: &str) -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();

    // Convert the folder path to a PathBuf for manipulation
    let folder_path = Path::new(folder);

    // Recursively visit all files and directories within the specified folder
    visit_folder(folder_path, PathBuf::new(), &mut paths);

    paths
}

/// Recursively visits subfolders and pushes files with certain extentions.
fn visit_folder(folder_path: &Path, relative_path: PathBuf, paths: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_file() {
                    // Check if the file has a .png extension
                    if let Some(ext) = path.extension() {
                        if ext == "png" {
                            // Construct the relative path to the file
                            let file_name =
                                path.file_name().unwrap().to_string_lossy().into_owned();
                            let mut file_path = relative_path.clone();
                            file_path.push(&file_name);

                            // Convert the path to a string
                            if let Some(file_str) = file_path.to_str() {
                                paths.push(file_str.to_string());
                            }
                        }
                    }
                } else if path.is_dir() {
                    // Recursively visit subdirectories
                    let mut next_relative_path = relative_path.clone();
                    next_relative_path.push(path.file_name().unwrap());

                    visit_folder(&path, next_relative_path, paths);
                }
            }
        }
    } else {
        // Handle read_dir error
        println!("Failed to read directory: {:?}", folder_path);
    }
}

/// sprite_sheet_states is an array holding tuples which hold the index of a beginning and end tile for a certain animation.
/// The order of animations are dormant:attack:dead.
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

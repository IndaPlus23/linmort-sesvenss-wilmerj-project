use bevy::prelude::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// SceneAssets stores handles for assets used in the scene.
#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub enemy: Handle<Scene>,
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
    let texture_paths = texture_paths("assets\\textures\\");

    *scene_assets = SceneAssets {
        enemy: asset_server.load(""),
        textures: load_textures_from_folder(texture_paths.clone(), asset_server),
        texture_paths: texture_paths,
    }
}

/// Loads folder of textures and upgrades into handle of image.
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

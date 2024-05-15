use bevy::prelude::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// SceneAssets stores handles for assets used in the scene.
#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub enemy: Handle<Scene>,
    pub hud: Vec<Handle<Image>>,
    pub cubemaps: Vec<Handle<Image>>,
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
pub fn load_assets(mut scene_assets: ResMut<SceneAssets>, mut asset_server: Res<AssetServer>) {
    let hud_paths = texture_paths("assets\\textures\\hud\\");
    let cubemap_paths = texture_paths("assets\\textures\\cubemap\\");
    let texture_paths = texture_paths("assets\\textures\\flats\\");

    *scene_assets = SceneAssets {
        enemy: asset_server.load(""),
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
        texture_paths: texture_paths,
    }
}

/// Loads folder of textures and upgrades into handle of image.
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

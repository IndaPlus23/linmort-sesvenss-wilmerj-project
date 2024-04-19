use bevy::asset::LoadedAsset;
use bevy::prelude::*;

const SAMPLE_ASSETS: Vec<String> = Vec::new();

/// SceneAssets stores handles for assets used in the scene.
#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub enemy: Handle<Scene>,
    pub textures: Vec<Handle<Image>>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>().add_systems(Startup, load_assets);
    }
}

/// Loads assets from asset folder and populates AssetScene, making them available
/// for usage without having multiple handles reference various copies of the same asset.
fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        enemy: asset_server.load("models/Rock.glb#Scene0"),
        textures: load_textures_from_folder(SAMPLE_ASSETS, asset_server),
    }
}

/// Loads folder of textures and upgrades into handle of image
fn load_textures_from_folder(assets: Vec<String>, asset_server: Res<AssetServer>) -> Vec<Handle<Image>> {
    let mut image_handles: Vec<Handle<Image>> = Vec::new();

    for asset in assets.iter() {
        // Unchecked loading
        let asset: Handle<Image> = asset_server.load(format!("{}{}", "textures/", asset));
        image_handles.push(asset);
    }

    return image_handles
}

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
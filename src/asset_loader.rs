use bevy::prelude::*;

/// SceneAssets stores handles for assets used in the scene.
#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub enemy: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>().add_systems(Startup, load_assets);
    }
}

/// load_assets loads assets from asset folder and populates AssetScene, making them available
/// for usage without having multiple handles reference various copies of the same asset.
fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        enemy: asset_server.load("models/Rock.glb#Scene0"),
    }
}

/// Loads a spritesheet given number of columns and rows
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
    let texture = asset_server.load("spritesheet.png");
    let layout = TextureAtlasLayout::from_grid(tile_size, columns, rows, None, None);
    texture_atlas_layouts.add(layout)
}
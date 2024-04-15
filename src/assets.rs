use bevy::asset::{AssetServer, Handle, LoadedFolder};
use bevy::prelude::{Commands, Res, Resource};
use bevy::text::Font;

#[derive(Resource)]
struct UiFont(Handle<Font>);
#[derive(Resource)]
struct Sprites(Handle<LoadedFolder>);
#[derive(Resource)]
struct Audio(Handle<LoadedFolder>);

/// Initialize all assets.
pub fn load_assets(
    mut commands: Commands,
    server: Res<AssetServer>
) {
    let handle: Handle<Font> = server.load("menu_font.ttf");

    commands.insert_resource(UiFont(handle));
    commands.insert_resource(Sprites(server.load_folder("images")));
    commands.insert_resource(Audio(server.load_folder("audio")));
}

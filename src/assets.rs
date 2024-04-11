use bevy::asset::{AssetServer, Handle};
use bevy::audio::AudioSource;
use bevy::prelude::{Commands, Res, Resource};
use bevy::render::render_asset::ExtractedAssets;
use bevy::text::Font;
use bevy::render::texture::Image;

#[derive(Resource)]
struct UiFont(Handle<Font>);
#[derive(Resource)]
struct Sprites(Vec<Handle<Image>>);
#[derive(Resource)]
struct Audio(Handle<AudioSource>);

/// Used to load UI font
fn load_ui_font(
    mut commands: Commands,
    server: Res<AssetServer>
) {
    let handle: Handle<Font> = server.load("menu_font.ttf");
    commands.insert_resource(UiFont(handle));
}

/// Used to load sprites (images)
fn load_sprites(
    mut commands: Commands,
    server: Res<AssetServer>
) {
    if let Ok(handles) = server.load_folder("assets/images") {
        commands.insert_resource(Sprites(handles))
    }
}

/// Used to load audio files
fn load_audio(
    mut commands: Commands,
    server: Res<AssetServer>
) {
    if let Ok(handles) = server.load_folder("assets/audio") {
        commands.insert_resource(Audio(handles))
    }
}
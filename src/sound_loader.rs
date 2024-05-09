use bevy::prelude::*;
use bevy::audio::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn play_background_audio(
    asset_server: Res<AssetServer>,
    mut commands: Commands
) {
    // Create an entity dedicated to playing our background music
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds\\02.-Cultist-Base.ogg"),
        settings: PlaybackSettings::LOOP,
    });
}

// increase and decrease the volyme
pub fn volume_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    music_box_query: Query<&AudioSink>
) {
    if let Ok(sink) = music_box_query.get_single() {
        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            sink.set_volume(sink.volume() + 0.1);
        } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            sink.set_volume(sink.volume() - 0.1);
        }
    }
}
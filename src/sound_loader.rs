use bevy::prelude::*;
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
use bevy::prelude::*;
use bevy::audio::*;
use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
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

#[derive(Component)]
pub struct MyMusic;

pub fn update_speed(music_controller: Query<&AudioSink, With<MyMusic>>, time: Res<Time>) {
    if let Ok(sink) = music_controller.get_single() {
        sink.set_speed(((time.elapsed_seconds() / 5.0).sin() + 1.0).max(0.1));
    }
}

pub fn pause(keyboard_input: Res<ButtonInput<KeyCode>>, music_controller: Query<&AudioSink, With<MyMusic>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok(sink) = music_controller.get_single() {
            sink.toggle();
        }
    }
}

pub fn volume(keyboard_input: Res<ButtonInput<KeyCode>>, music_controller: Query<&AudioSink, With<MyMusic>>) {
    if let Ok(sink) = music_controller.get_single() {
        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            sink.set_volume(sink.volume() + 0.1);
        } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            sink.set_volume(sink.volume() - 0.1);
        }
        println!("{:?}", sink.volume())
    }
}
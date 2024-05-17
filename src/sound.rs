use bevy::audio::Volume;
use bevy::prelude::*;

#[derive(Component)]
pub struct BackgroundSong;

// Play background song
pub fn play_background_audio(
    asset_server: &mut Res<AssetServer>,
    commands: &mut Commands,
    path: String,
) {
    commands.spawn((
        BackgroundSong,
        AudioBundle {
            source: asset_server.load(path),
            settings: PlaybackSettings::LOOP.with_volume(Volume::new(0.2)),
        },
    ));
}

// Pause music
pub fn pause_audio(music_controller: Query<&AudioSink>) {
    if let Ok(sink) = music_controller.get_single() {
        sink.toggle();
    }
}

pub fn increase_volume(music_controller: Query<&AudioSink>) {
    if let Ok(sink) = music_controller.get_single() {
        sink.set_volume(sink.volume() + 0.05);
    }
}

pub fn decrease_volume(music_controller: Query<&AudioSink>) {
    if let Ok(sink) = music_controller.get_single() {
        if sink.volume() - 0.1 > 0. {
            sink.set_volume(sink.volume() - 0.05);
        } else {
            sink.set_volume(0.)
        }
    }
}

/*
EXAMPLE:

you need these ->

    asset_server: Res<AssetServer>,
    mut commands: Commands

then use the function like this (shotgun.ogg must be a file in assets/sounds)->

    play_audio(asset_server,commands, "shotgun.ogg")

*/

pub fn play_audio(asset_server: Res<AssetServer>, mut commands: Commands, path: &str) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds\\".to_owned() + path),
        settings: PlaybackSettings::ONCE.with_volume(Volume::new(0.3)),
        ..default()
    });
}

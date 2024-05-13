use bevy::prelude::*;

// startup background song
pub fn play_background_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds\\02.-Cultist-Base.ogg"),
        settings: PlaybackSettings::LOOP,
    });
}

// Pause music
pub fn pause_audio(music_controller: Query<&AudioSink>) {
    if let Ok(sink) = music_controller.get_single() {
        sink.toggle();
    }
}

pub fn increase_audio(music_controller: Query<&AudioSink>) {
    if let Ok(sink) = music_controller.get_single() {
        sink.set_volume(sink.volume() + 0.05);   
    }
}

pub fn decrease_audio(music_controller: Query<&AudioSink>) {
    if let Ok(sink) = music_controller.get_single() {
        if sink.volume() - 0.1 > 0. {
            sink.set_volume(sink.volume() - 0.05);
        } else {
            sink.set_volume(0.)
        }   
    }
}

pub fn play_audio(asset_server: Res<AssetServer>, mut commands: Commands, path: &str) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds\\".to_owned() + path),
        ..default()
    });
}
use bevy::prelude::*;
use kgrs_config::Config;

/// Play music
pub fn setup_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let volume = Config::load().music_volume;
    if 0. < volume {
        audio.play_with_settings(
            asset_server.load("musics/Fugue-in-Glop-de-Bal.ogg"), // TODO: more musics
            PlaybackSettings::LOOP.with_volume(volume / 50.),
        );
        info!("Playing music");
    } else {
        info!("Music is muted");
    }
}

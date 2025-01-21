use super::*;
use bevy::audio::Volume;

#[derive(Resource)]
pub(crate) struct AudioAssets {
    pub(crate) fall_sound: Handle<AudioSource>,
    pub(crate) flap_sound: Handle<AudioSource>,
    pub(crate) score_sound: Handle<AudioSource>,
    pub(crate) smack_sound: Handle<AudioSource>,
    pub(crate) swoosh_sound: Handle<AudioSource>,
    pub(crate) music: Handle<AudioSource>,
}

pub(crate) fn insert_audio_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut volume: ResMut<GlobalVolume>,
) {
    volume.volume = Volume::new(GLOBAL_SOUND_VOLUME);
    commands.insert_resource(AudioAssets {
        fall_sound: asset_server.load(FALL_SOUND),
        flap_sound: asset_server.load(FLAP_SOUND),
        score_sound: asset_server.load(SCORE_SOUND),
        smack_sound: asset_server.load(SMACK_SOUND),
        swoosh_sound: asset_server.load(SWOOSH_SOUND),
        music: asset_server.load(MUSIC),
    });
}

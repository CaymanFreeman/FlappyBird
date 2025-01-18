use crate::game::setup_game_manager;
use bevy::app::{App, Plugin, Startup};
use bevy::asset::{AssetServer, Handle};
use bevy::audio::AudioSource;
use bevy::color::Color;
use bevy::image::Image;
use bevy::prelude::{ClearColor, Commands, Res, Resource};

pub(crate) const SPRITE_SCALE: f32 = 4.0;
const PIPE_SPRITE: &str = "sprites/pipe.png";
const PLAYER_SPRITE: &str = "sprites/bird.png";

pub(crate) const PLAYER_SPRITE_Z: f32 = 1.0;
pub(crate) const PIPE_SPRITE_Z: f32 = 0.0;

const BACKGROUND_COLOR: [f32; 3] = [0.502, 0.702, 0.8]; // #80b3cc

const FALL_SOUND: &str = "sounds/fall.ogg";
const FLAP_SOUND: &str = "sounds/flap.ogg";
const SCORE_SOUND: &str = "sounds/score.ogg";
const SMACK_SOUND: &str = "sounds/smack.ogg";
const SWOOSH_SOUND: &str = "sounds/swoosh.ogg";
const MUSIC: &str = "sounds/music.ogg";

#[derive(Resource)]
pub(crate) struct SpriteManager {
    pub pipe_sprite: Handle<Image>,
    pub player_sprite: Handle<Image>,
}

#[derive(Resource)]
pub(crate) struct AudioManager {
    pub fall_sound: Handle<AudioSource>,
    pub flap_sound: Handle<AudioSource>,
    pub score_sound: Handle<AudioSource>,
    pub smack_sound: Handle<AudioSource>,
    pub swoosh_sound: Handle<AudioSource>,
    pub music: Handle<AudioSource>,
}

pub(crate) struct GameAssetPlugin;

impl Plugin for GameAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                setup_game_manager,
                setup_sprite_manager,
                setup_audio_manager,
            ),
        );
    }
}

pub(crate) fn setup_sprite_manager(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ClearColor(Color::srgb_from_array(BACKGROUND_COLOR)));
    commands.insert_resource(SpriteManager {
        pipe_sprite: asset_server.load(PIPE_SPRITE),
        player_sprite: asset_server.load(PLAYER_SPRITE),
    });
}

pub(crate) fn setup_audio_manager(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AudioManager {
        fall_sound: asset_server.load(FALL_SOUND),
        flap_sound: asset_server.load(FLAP_SOUND),
        score_sound: asset_server.load(SCORE_SOUND),
        smack_sound: asset_server.load(SMACK_SOUND),
        swoosh_sound: asset_server.load(SWOOSH_SOUND),
        music: asset_server.load(MUSIC),
    });
}

use super::*;

mod audio;
mod fonts;
mod sprites;

// Re-exports
pub(crate) use audio::*;
pub(crate) use fonts::*;
pub(crate) use sprites::*;

// Fonts
const BUTTON_FONT: &str = "fonts/mini_pixel-7.ttf";
const TITLE_FONT: &str = "fonts/light_pixel-7.ttf";

// Sprites
pub(crate) const SPRITE_SCALE: f32 = 4.0;
const PIPE_SPRITE: &str = "sprites/pipe.png";
const PLAYER_SPRITE: &str = "sprites/bird.png";

pub(crate) const PLAYER_SPRITE_Z: f32 = 1.0;
pub(crate) const PIPE_SPRITE_Z: f32 = 0.0;

const GAME_BACKGROUND_COLOR: [f32; 3] = [0.565, 0.855, 1.0]; // #90daff

// Audio
const FALL_SOUND: &str = "sounds/fall.ogg";
const FLAP_SOUND: &str = "sounds/flap.ogg";
const SCORE_SOUND: &str = "sounds/score.ogg";
const SMACK_SOUND: &str = "sounds/smack.ogg";
const SWOOSH_SOUND: &str = "sounds/swoosh.ogg";
const MUSIC: &str = "sounds/music.ogg";

const GLOBAL_SOUND_VOLUME: f32 = 0.3;
pub(crate) const FALL_SOUND_VOLUME: f32 = 1.0;
pub(crate) const FLAP_SOUND_VOLUME: f32 = 1.0;
pub(crate) const SCORE_SOUND_VOLUME: f32 = 1.0;
pub(crate) const SMACK_SOUND_VOLUME: f32 = 1.0;
pub(crate) const SWOOSH_SOUND_VOLUME: f32 = 1.0;
pub(crate) const MUSIC_VOLUME: f32 = 1.0;

pub(crate) struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                insert_audio_assets,
                insert_sprite_assets,
                insert_font_assets,
            ),
        );
    }
}

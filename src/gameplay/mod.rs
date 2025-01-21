use crate::assets::*;
use crate::ui::Score;
use bevy::prelude::*;

mod pipes;
mod player;

// Re-exports
pub(crate) use pipes::*;
pub(crate) use player::*;

// Pipes
const PIPE_AMOUNT: i32 = 4;
const PIPE_WIDTH: f32 = 18.0;
const PIPE_HEIGHT: f32 = 144.0;
const PIPE_GAP_SIZE: f32 = 15.0;
const PIPE_VERTICAL_OFFSET: f32 = 30.0;
pub(crate) const PIPE_SPACING: f32 = 60.0;
pub(crate) const PIPE_SPEED: f32 = 150.0;

const PIPE_HALF_WIDTH: f32 = (PIPE_WIDTH * SPRITE_SCALE) / 2.0;
const PIPE_HALF_HEIGHT: f32 = (PIPE_HEIGHT * SPRITE_SCALE) / 2.0;
const PIPE_VERTICAL_CENTER: f32 = (PIPE_HEIGHT / 2.0 + PIPE_GAP_SIZE) * SPRITE_SCALE;

const PIPE_DIRECTION_UP: f32 = 1.0;
const PIPE_DIRECTION_DOWN: f32 = -1.0;

// Player
const PLAYER_WIDTH: f32 = 12.0;
const PLAYER_HEIGHT: f32 = 8.0;
const PLAYER_COLLISION_RATIO: f32 = 0.3;

const PLAYER_FLAP_FORCE: f32 = 500.0;
const PLAYING_GRAVITY_STRENGTH: f32 = 1800.0;
const ANIMATION_GRAVITY_STRENGTH: f32 = 750.0;
const VELOCITY_TO_ROTATION_RATIO: f32 = 13.0;

const FLAP_KEY: KeyCode = KeyCode::Space;

const FALL_SOUND_DELAY: f32 = 0.5;
const FALL_RESET_DELAY: f32 = 1.75;

pub(crate) struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            update_pipe_transforms.run_if(in_state(PlayerState::Flapping)),
        )
        .add_systems(Startup, |mut commands: Commands| {
            commands.insert_resource(Score(0));
        })
        .init_state::<PlayerState>()
        .add_systems(
            Update,
            (
                handle_frozen_toggle.run_if(in_state(PlayerState::WaitingToStart)),
                handle_fall_sound_delay_timer.run_if(in_state(PlayerState::WaitingToFall)),
                handle_fall_animation.run_if(in_state(PlayerState::Falling)),
                handle_player_input.run_if(in_state(PlayerState::Flapping)),
                handle_fall_reset_delay_timer.run_if(in_state(PlayerState::Falling)),
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                update_player_transform.run_if(in_state(PlayerState::Flapping)),
                handle_player_collision.run_if(in_state(PlayerState::Flapping)),
            )
                .chain(),
        );
    }
}

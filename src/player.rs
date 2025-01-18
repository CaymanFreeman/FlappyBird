use crate::assets::{AudioManager, PLAYER_SPRITE_Z, SPRITE_SCALE};
use crate::game::{FallDelayTimer, GameManager, GameState};
use crate::pipe::{Pipe, PIPE_HEIGHT, PIPE_WIDTH};
use bevy::asset::Handle;
use bevy::audio::AudioPlayer;
use bevy::image::Image;
use bevy::input::ButtonInput;
use bevy::math::{Quat, Rect, Vec2, Vec3};
use bevy::prelude::{
    Bundle, Commands, Component, KeyCode, Mut, NextState, Query, Res, ResMut, States, Transform,
    With, Without,
};
use bevy::sprite::Sprite;
use bevy::time::Time;

const PLAYER_WIDTH: f32 = 12.0;
const PLAYER_HEIGHT: f32 = 8.0;
const PLAYER_COLLISION_RATIO: f32 = 0.3;

pub const FLAP_FORCE: f32 = 500.0;
const GRAVITY_STRENGTH: f32 = 1800.0;
const ROTATION_RATIO: f32 = 13.0;

pub const FLAP_KEY: KeyCode = KeyCode::Space;

#[derive(Component)]
pub struct Player {
    pub velocity: f32,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    sprite: Sprite,
    transform: Transform,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerState {
    #[default]
    WaitingToStart,
    WaitingToFall,
    Falling,
    Flapping,
}

impl PlayerBundle {
    pub fn new(player_sprite: &Handle<Image>) -> PlayerBundle {
        PlayerBundle {
            sprite: Sprite {
                image: player_sprite.clone(),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, PLAYER_SPRITE_Z).with_scale(Vec3::new(
                SPRITE_SCALE,
                SPRITE_SCALE,
                1.0,
            )),
            player: Player { velocity: 0.0 },
        }
    }
}

pub fn update_player_transform(
    mut player_transform_query: Query<(&mut Player, &mut Transform)>,
    time: Res<Time>,
) {
    if let Ok((mut player, mut player_transform)) = player_transform_query.get_single_mut() {
        apply_player_gravity(&mut player, &mut player_transform, &time);
        apply_player_rotation(&mut player, &mut player_transform);
    }
}

fn apply_player_gravity(
    player: &mut Mut<Player>,
    player_transform: &mut Mut<Transform>,
    time: &Res<Time>,
) {
    player.velocity -= time.delta_secs() * GRAVITY_STRENGTH;
    player_transform.translation.y += player.velocity * time.delta_secs();
}

fn apply_player_rotation(player: &mut Mut<Player>, player_transform: &mut Mut<Transform>) {
    player_transform.rotation = Quat::from_axis_angle(
        Vec3::Z,
        f32::clamp(player.velocity / ROTATION_RATIO, -90.0, 90.0).to_radians(),
    );
}

pub fn handle_player_input(
    mut commands: Commands,
    mut player_query: Query<&mut Player, Without<Pipe>>,
    keys: Res<ButtonInput<KeyCode>>,
    audio_manager: Res<AudioManager>,
) {
    if keys.just_pressed(FLAP_KEY) {
        commands.spawn(AudioPlayer::new(audio_manager.flap_sound.clone()));
        if let Ok(mut player) = player_query.get_single_mut() {
            player.velocity = FLAP_FORCE;
        }
    }
}

pub fn handle_player_collision(
    mut commands: Commands,
    player_transform_query: Query<&Transform, With<Player>>,
    pipe_transform_query: Query<&Transform, With<Pipe>>,
    game_manager: Res<GameManager>,
    audio_manager: Res<AudioManager>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    if let Ok(player_transform) = player_transform_query.get_single() {
        let player_has_collision = player_pipe_collision(&player_transform, pipe_transform_query)
            || player_screen_collision(&player_transform, &game_manager);

        if player_has_collision {
            commands.spawn(AudioPlayer::new(audio_manager.smack_sound.clone()));
            next_player_state.set(PlayerState::WaitingToFall);
            commands.spawn(FallDelayTimer::new());
        }
    }
}

fn player_pipe_collision(
    player_transform: &Transform,
    pipe_transform_query: Query<&Transform, With<Pipe>>,
) -> bool {
    let player_radius = (PLAYER_WIDTH.min(PLAYER_HEIGHT) * SPRITE_SCALE) * PLAYER_COLLISION_RATIO;
    let player_center = player_transform.translation.truncate();

    for pipe_transform in pipe_transform_query.iter() {
        let pipe_rect = Rect {
            min: Vec2::new(
                pipe_transform.translation.x - (PIPE_WIDTH * SPRITE_SCALE) / 2.0,
                pipe_transform.translation.y - (PIPE_HEIGHT * SPRITE_SCALE) / 2.0,
            ),
            max: Vec2::new(
                pipe_transform.translation.x + (PIPE_WIDTH * SPRITE_SCALE) / 2.0,
                pipe_transform.translation.y + (PIPE_HEIGHT * SPRITE_SCALE) / 2.0,
            ),
        };

        let closest = Vec2::new(
            player_center.x.clamp(pipe_rect.min.x, pipe_rect.max.x),
            player_center.y.clamp(pipe_rect.min.y, pipe_rect.max.y),
        );

        if player_center.distance(closest) < player_radius {
            return true;
        }
    }
    false
}

fn player_screen_collision(player_transform: &Transform, game_manager: &Res<GameManager>) -> bool {
    player_transform.translation.y <= -game_manager.window_dimensions.y / 2.0
        || player_transform.translation.y >= game_manager.window_dimensions.y / 2.0
}

pub fn handle_fall_animation(
    mut player_transform_query: Query<(&mut Player, &mut Transform)>,
    time: Res<Time>,
    game_manager: Res<GameManager>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok((mut player, mut player_transform)) = player_transform_query.get_single_mut() {
        apply_player_gravity(&mut player, &mut player_transform, &time);
        apply_player_rotation(&mut player, &mut player_transform);
        if player_screen_collision(&player_transform, &game_manager) {
            next_game_state.set(GameState::Menu);
        }
    }
}

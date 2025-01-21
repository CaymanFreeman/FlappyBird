use super::*;
use crate::app::{AppState, WINDOW_MAX_Y, WINDOW_MIN_Y};
use crate::ui::ScoreTimer;
use bevy::audio::Volume;
use pipes::Pipe;
use std::time::Duration;

#[derive(Component)]
pub(crate) struct Player {
    velocity: f32,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum PlayerState {
    #[default]
    WaitingToStart,
    WaitingToFall,
    Falling,
    Flapping,
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    sprite: Sprite,
    transform: Transform,
}

#[derive(Component)]
pub(crate) struct FallDelayTimer(Timer);

#[derive(Component)]
pub(crate) struct ResetDelayTimer(Timer);

impl PlayerBundle {
    fn new(player_sprite: &Handle<Image>) -> PlayerBundle {
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

impl FallDelayTimer {
    fn new() -> FallDelayTimer {
        FallDelayTimer(Timer::new(
            Duration::from_secs_f32(FALL_SOUND_DELAY),
            TimerMode::Once,
        ))
    }
}

impl ResetDelayTimer {
    fn new() -> ResetDelayTimer {
        ResetDelayTimer(Timer::new(
            Duration::from_secs_f32(FALL_RESET_DELAY),
            TimerMode::Once,
        ))
    }
}

pub(crate) fn spawn_player(mut commands: Commands, sprites: Res<SpriteAssets>) {
    commands.spawn(PlayerBundle::new(&sprites.player_sprite));
}

pub(crate) fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    for entity in player_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub(crate) fn update_player_transform(
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
    player.velocity -= time.delta_secs() * PLAYING_GRAVITY_STRENGTH;
    player_transform.translation.y += player.velocity * time.delta_secs();
}

fn apply_player_animation_gravity(
    player: &mut Mut<Player>,
    player_transform: &mut Mut<Transform>,
    time: &Res<Time>,
) {
    player.velocity -= time.delta_secs() * ANIMATION_GRAVITY_STRENGTH;
    player_transform.translation.y += player.velocity * time.delta_secs();
}

fn apply_player_rotation(player: &mut Mut<Player>, player_transform: &mut Mut<Transform>) {
    player_transform.rotation = Quat::from_axis_angle(
        Vec3::Z,
        f32::clamp(player.velocity / VELOCITY_TO_ROTATION_RATIO, -90.0, 90.0).to_radians(),
    );
}

pub(crate) fn handle_player_input(
    mut commands: Commands,
    mut player_query: Query<&mut Player>,
    keys: Res<ButtonInput<KeyCode>>,
    audio: Res<AudioAssets>,
) {
    if keys.just_pressed(FLAP_KEY) {
        commands.spawn((
            AudioPlayer::new(audio.flap_sound.clone()),
            PlaybackSettings {
                volume: Volume::new(FLAP_SOUND_VOLUME),
                ..Default::default()
            },
        ));
        if let Ok(mut player) = player_query.get_single_mut() {
            player.velocity = PLAYER_FLAP_FORCE;
        }
    }
}

pub(crate) fn handle_player_collision(
    mut commands: Commands,
    player_transform_query: Query<&Transform, With<Player>>,
    pipe_transform_query: Query<&Transform, With<Pipe>>,
    mut player_query: Query<&mut Player>,
    score_timer_query: Query<Entity, With<ScoreTimer>>,
    audio: Res<AudioAssets>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    if let Ok(player_transform) = player_transform_query.get_single() {
        let pipe_collision = player_pipe_collision(&player_transform, pipe_transform_query);
        let screen_collision = player_screen_collision(&player_transform);

        if pipe_collision || screen_collision {
            if let Ok(score_display) = score_timer_query.get_single() {
                commands.entity(score_display).despawn();
            }
            if let Ok(mut player) = player_query.get_single_mut() {
                player.velocity = 0.0;
            }
            commands.spawn((
                AudioPlayer::new(audio.smack_sound.clone()),
                PlaybackSettings {
                    volume: Volume::new(SMACK_SOUND_VOLUME),
                    ..Default::default()
                },
            ));
            next_player_state.set(PlayerState::WaitingToFall);
            commands.spawn(FallDelayTimer::new());
        }
    }
}

fn player_pipe_collision(
    player_transform: &Transform,
    pipe_transform_query: Query<&Transform, With<Pipe>>,
) -> bool {
    for pipe_transform in pipe_transform_query.iter() {
        let too_far_right = pipe_transform.translation.x - PIPE_HALF_WIDTH_SCALED
            >= PLAYER_COLLISION_RADIUS_FACTORED;
        let too_far_left = pipe_transform.translation.x + PIPE_HALF_WIDTH_SCALED
            <= PLAYER_COLLISION_RADIUS_FACTORED;
        if too_far_right || too_far_left {
            continue;
        }

        if circle_rectangle_collision(
            player_transform,
            PLAYER_COLLISION_RADIUS_FACTORED,
            pipe_transform,
            PIPE_WIDTH_SCALED,
            PIPE_HEIGHT_SCALED,
        ) {
            return true;
        }
    }
    false
}

fn player_screen_collision(player_transform: &Transform) -> bool {
    player_transform.translation.y <= WINDOW_MIN_Y || player_transform.translation.y >= WINDOW_MAX_Y
}

pub(crate) fn handle_fall_sound_delay_timer(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FallDelayTimer)>,
    audio: Res<AudioAssets>,
    time: Res<Time>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    if let Ok((entity, mut delay_timer)) = query.get_single_mut() {
        if delay_timer.0.tick(time.delta()).just_finished() {
            next_player_state.set(PlayerState::Falling);
            commands.spawn(ResetDelayTimer::new());
            commands.spawn((
                AudioPlayer::new(audio.fall_sound.clone()),
                PlaybackSettings {
                    volume: Volume::new(FALL_SOUND_VOLUME),
                    ..Default::default()
                },
            ));
            commands.entity(entity).despawn();
        }
    }
}

pub(crate) fn handle_fall_reset_delay_timer(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ResetDelayTimer)>,
    time: Res<Time>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok((entity, mut delay_timer)) = query.get_single_mut() {
        if delay_timer.0.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
            next_app_state.set(AppState::RetryMenu);
        }
    }
}

pub(crate) fn handle_frozen_toggle(
    mut commands: Commands,
    mut player_query: Query<&mut Player, Without<Pipe>>,
    player_state: Res<State<PlayerState>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
    keys: Res<ButtonInput<KeyCode>>,
    audio: Res<AudioAssets>,
) {
    if keys.just_pressed(FLAP_KEY) {
        if let PlayerState::WaitingToStart = player_state.get() {
            commands.spawn(ScoreTimer::new());
            next_player_state.set(PlayerState::Flapping);
            commands.spawn((
                AudioPlayer::new(audio.flap_sound.clone()),
                PlaybackSettings {
                    volume: Volume::new(FLAP_SOUND_VOLUME),
                    ..Default::default()
                },
            ));
            if let Ok(mut player) = player_query.get_single_mut() {
                player.velocity = PLAYER_FLAP_FORCE;
            }
        }
    }
}

pub(crate) fn handle_fall_animation(
    mut player_transform_query: Query<(&mut Player, &mut Transform)>,
    time: Res<Time>,
) {
    if let Ok((mut player, mut player_transform)) = player_transform_query.get_single_mut() {
        apply_player_animation_gravity(&mut player, &mut player_transform, &time);
        apply_player_rotation(&mut player, &mut player_transform);
    }
}

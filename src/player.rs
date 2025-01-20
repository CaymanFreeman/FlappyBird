use crate::assets::{
    AudioManager, SpriteManager, FALL_SOUND_VOLUME, FLAP_SOUND_VOLUME, PLAYER_SPRITE_Z,
    SCORE_SOUND_VOLUME, SMACK_SOUND_VOLUME, SPRITE_SCALE,
};
use crate::game::{GameState, WindowManager};
use crate::menu::MenuSystems;
use crate::pipe::{
    Pipe, ScoreZone, PIPE_GAP_SIZE, PIPE_HALF_HEIGHT, PIPE_HALF_WIDTH, SCORE_ZONE_WIDTH,
};
use bevy::app::{App, FixedUpdate, Plugin, Startup, Update};
use bevy::asset::Handle;
use bevy::audio::{AudioPlayer, PlaybackSettings, Volume};
use bevy::image::Image;
use bevy::input::ButtonInput;
use bevy::math::{Quat, Rect, Vec2, Vec3};
use bevy::prelude::{
    in_state, AppExtStates, Bundle, Commands, Component, Entity, IntoSystemConfigs, KeyCode, Mut,
    NextState, Query, Res, ResMut, Resource, State, States, Timer, TimerMode, Transform, With,
    Without,
};
use bevy::sprite::Sprite;
use bevy::time::Time;
use std::time::Duration;

const PLAYER_WIDTH: f32 = 12.0;
const PLAYER_HEIGHT: f32 = 8.0;
const PLAYER_COLLISION_RATIO: f32 = 0.3;

pub(crate) const FLAP_FORCE: f32 = 500.0;
const GRAVITY_STRENGTH: f32 = 1800.0;
const ANIMATION_GRAVITY_STRENGTH: f32 = 750.0;
const ROTATION_RATIO: f32 = 13.0;

pub(crate) const FLAP_KEY: KeyCode = KeyCode::Space;

const FALL_SOUND_DELAY: f32 = 0.5;
const FALL_RESET_DELAY: f32 = 1.75;

#[derive(Component)]
pub(crate) struct Player {
    pub velocity: f32,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum PlayerState {
    #[default]
    WaitingToStart,
    WaitingToFall,
    Falling,
    Flapping,
}

#[derive(Resource)]
pub(crate) struct Score(pub u32);

#[derive(Bundle)]
pub(crate) struct PlayerBundle {
    player: Player,
    sprite: Sprite,
    transform: Transform,
}

impl PlayerBundle {
    pub(crate) fn new(player_sprite: &Handle<Image>) -> PlayerBundle {
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

pub(crate) fn spawn_player(mut commands: Commands, sprite_manager: Res<SpriteManager>) {
    commands.spawn(PlayerBundle::new(&sprite_manager.player_sprite));
}

pub(crate) fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    for entity in player_query.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Component)]
pub(crate) struct FallDelayTimer(Timer);

impl FallDelayTimer {
    pub(crate) fn new() -> FallDelayTimer {
        FallDelayTimer(Timer::new(
            Duration::from_secs_f32(FALL_SOUND_DELAY),
            TimerMode::Once,
        ))
    }
}

#[derive(Component)]
pub(crate) struct ResetDelayTimer(Timer);

impl ResetDelayTimer {
    pub(crate) fn new() -> ResetDelayTimer {
        ResetDelayTimer(Timer::new(
            Duration::from_secs_f32(FALL_RESET_DELAY),
            TimerMode::Once,
        ))
    }
}

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut commands: Commands| {
            commands.insert_resource(Score(0));
        });
        app.init_state::<PlayerState>();
        app.add_systems(
            Update,
            (
                handle_frozen_toggle.run_if(in_state(PlayerState::WaitingToStart)),
                update_fall_sound_delay_timer.run_if(in_state(PlayerState::WaitingToFall)),
                handle_fall_animation.run_if(in_state(PlayerState::Falling)),
                handle_player_input.run_if(in_state(PlayerState::Flapping)),
                update_fall_reset_delay_timer.run_if(in_state(PlayerState::Falling)),
            ),
        );
        app.add_systems(
            FixedUpdate,
            (
                update_player_transform.run_if(in_state(PlayerState::Flapping)),
                handle_player_collision.run_if(in_state(PlayerState::Flapping)),
                handle_score.run_if(in_state(PlayerState::Flapping)),
            )
                .chain(),
        );
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

pub(crate) fn apply_player_gravity(
    player: &mut Mut<Player>,
    player_transform: &mut Mut<Transform>,
    time: &Res<Time>,
) {
    player.velocity -= time.delta_secs() * GRAVITY_STRENGTH;
    player_transform.translation.y += player.velocity * time.delta_secs();
}

pub(crate) fn apply_player_animation_gravity(
    player: &mut Mut<Player>,
    player_transform: &mut Mut<Transform>,
    time: &Res<Time>,
) {
    player.velocity -= time.delta_secs() * ANIMATION_GRAVITY_STRENGTH;
    player_transform.translation.y += player.velocity * time.delta_secs();
}

pub(crate) fn apply_player_rotation(
    player: &mut Mut<Player>,
    player_transform: &mut Mut<Transform>,
) {
    player_transform.rotation = Quat::from_axis_angle(
        Vec3::Z,
        f32::clamp(player.velocity / ROTATION_RATIO, -90.0, 90.0).to_radians(),
    );
}

pub(crate) fn handle_player_input(
    mut commands: Commands,
    mut player_query: Query<&mut Player>,
    keys: Res<ButtonInput<KeyCode>>,
    audio_manager: Res<AudioManager>,
) {
    if keys.just_pressed(FLAP_KEY) {
        commands.spawn((
            AudioPlayer::new(audio_manager.flap_sound.clone()),
            PlaybackSettings {
                volume: Volume::new(FLAP_SOUND_VOLUME),
                ..Default::default()
            },
        ));
        if let Ok(mut player) = player_query.get_single_mut() {
            player.velocity = FLAP_FORCE;
        }
    }
}

pub(crate) fn handle_score(
    mut commands: Commands,
    player_transform_query: Query<&Transform, With<Player>>,
    pipe_transform_query: Query<(Entity, &Transform), (With<Pipe>, With<ScoreZone>)>,
    audio_manager: Res<AudioManager>,
    mut score: ResMut<Score>,
    menu_systems: Res<MenuSystems>,
) {
    if let Ok(player_transform) = player_transform_query.get_single() {
        for (entity, pipe_transform) in pipe_transform_query.iter() {
            let pipe_min_x = pipe_transform.translation.x - SCORE_ZONE_WIDTH / 2.0;
            let pipe_max_x = pipe_transform.translation.x + SCORE_ZONE_WIDTH / 2.0;
            let pipe_min_y = pipe_transform.translation.y + PIPE_HALF_HEIGHT;
            let pipe_max_y =
                pipe_transform.translation.y + PIPE_HALF_HEIGHT + PIPE_GAP_SIZE * SPRITE_SCALE;

            let player_min_x = player_transform.translation.x - PLAYER_WIDTH * SPRITE_SCALE;
            let player_max_x = player_transform.translation.x + PLAYER_WIDTH * SPRITE_SCALE;
            let player_min_y = player_transform.translation.y - PLAYER_HEIGHT * SPRITE_SCALE;
            let player_max_y = player_transform.translation.y + PLAYER_HEIGHT * SPRITE_SCALE;

            if player_min_x < pipe_max_x
                && player_max_x > pipe_min_x
                && player_min_y < pipe_max_y
                && player_max_y > pipe_min_y
            {
                commands.spawn((
                    AudioPlayer::new(audio_manager.score_sound.clone()),
                    PlaybackSettings {
                        volume: Volume::new(SCORE_SOUND_VOLUME),
                        ..Default::default()
                    },
                ));
                commands.entity(entity).remove::<ScoreZone>();
                score.0 += 1;
                commands.run_system(menu_systems.update_score_system_id);
            }
        }
    }
}

pub(crate) fn handle_player_collision(
    mut commands: Commands,
    player_transform_query: Query<&Transform, With<Player>>,
    pipe_transform_query: Query<&Transform, With<Pipe>>,
    mut player_query: Query<&mut Player>,
    game_manager: Res<WindowManager>,
    audio_manager: Res<AudioManager>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    if let Ok(player_transform) = player_transform_query.get_single() {
        let player_has_collision = player_pipe_collision(&player_transform, pipe_transform_query)
            || player_screen_collision(&player_transform, &game_manager);

        if player_has_collision {
            if let Ok(mut player) = player_query.get_single_mut() {
                player.velocity = 0.0;
            }
            commands.spawn((
                AudioPlayer::new(audio_manager.smack_sound.clone()),
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

pub(crate) fn player_pipe_collision(
    player_transform: &Transform,
    pipe_transform_query: Query<&Transform, With<Pipe>>,
) -> bool {
    let player_radius = (PLAYER_WIDTH.min(PLAYER_HEIGHT) * SPRITE_SCALE) * PLAYER_COLLISION_RATIO;
    let player_center = player_transform.translation.truncate();

    for pipe_transform in pipe_transform_query.iter() {
        let pipe_rect = Rect {
            min: Vec2::new(
                pipe_transform.translation.x - PIPE_HALF_WIDTH,
                pipe_transform.translation.y - PIPE_HALF_HEIGHT,
            ),
            max: Vec2::new(
                pipe_transform.translation.x + PIPE_HALF_WIDTH,
                pipe_transform.translation.y + PIPE_HALF_HEIGHT,
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

pub(crate) fn player_screen_collision(
    player_transform: &Transform,
    game_manager: &Res<WindowManager>,
) -> bool {
    player_transform.translation.y <= -game_manager.window_dimensions.y / 2.0
        || player_transform.translation.y >= game_manager.window_dimensions.y / 2.0
}

pub(crate) fn update_fall_sound_delay_timer(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FallDelayTimer)>,
    audio_manager: Res<AudioManager>,
    time: Res<Time>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    if let Ok((entity, mut delay_timer)) = query.get_single_mut() {
        if delay_timer.0.tick(time.delta()).just_finished() {
            next_player_state.set(PlayerState::Falling);
            commands.spawn(ResetDelayTimer::new());
            commands.spawn((
                AudioPlayer::new(audio_manager.fall_sound.clone()),
                PlaybackSettings {
                    volume: Volume::new(FALL_SOUND_VOLUME),
                    ..Default::default()
                },
            ));
            commands.entity(entity).despawn();
        }
    }
}

pub(crate) fn update_fall_reset_delay_timer(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ResetDelayTimer)>,
    time: Res<Time>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok((entity, mut delay_timer)) = query.get_single_mut() {
        if delay_timer.0.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
            next_game_state.set(GameState::RetryMenu);
        }
    }
}

pub(crate) fn handle_frozen_toggle(
    mut commands: Commands,
    mut player_query: Query<&mut Player, Without<Pipe>>,
    player_state: Res<State<PlayerState>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
    keys: Res<ButtonInput<KeyCode>>,
    audio_manager: Res<AudioManager>,
) {
    if keys.just_pressed(FLAP_KEY) {
        if let PlayerState::WaitingToStart = player_state.get() {
            next_player_state.set(PlayerState::Flapping);
            commands.spawn((
                AudioPlayer::new(audio_manager.flap_sound.clone()),
                PlaybackSettings {
                    volume: Volume::new(FLAP_SOUND_VOLUME),
                    ..Default::default()
                },
            ));
            if let Ok(mut player) = player_query.get_single_mut() {
                player.velocity = FLAP_FORCE;
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

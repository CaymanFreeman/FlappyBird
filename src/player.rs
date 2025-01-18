use crate::assets::{AudioManager, SpriteManager, SPRITE_SCALE};
use crate::game::GameManager;
use crate::pipe::{spawn_pipes, Pipe, PIPE_HEIGHT, PIPE_WIDTH};
use bevy::asset::Handle;
use bevy::audio::AudioPlayer;
use bevy::image::Image;
use bevy::input::ButtonInput;
use bevy::math::{Quat, Rect, Vec2, Vec3};
use bevy::prelude::{
    Bundle, Commands, Component, Entity, KeyCode, Mut, Query, Res, Transform, With, Without,
};
use bevy::sprite::Sprite;
use bevy::time::Time;

const PLAYER_WIDTH: f32 = 12.0;
const PLAYER_HEIGHT: f32 = 8.0;
const PLAYER_COLLISION_RATIO: f32 = 0.3;

const FLAP_FORCE: f32 = 500.0;
const GRAVITY_STRENGTH: f32 = 2000.0;
const ROTATION_RATIO: f32 = 17.0;

const FLAP_KEY: KeyCode = KeyCode::Space;

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

impl PlayerBundle {
    pub fn new(player_sprite: &Handle<Image>) -> PlayerBundle {
        PlayerBundle {
            sprite: Sprite {
                image: player_sprite.clone(),
                ..Default::default()
            },
            transform: Transform::IDENTITY.with_scale(Vec3::splat(SPRITE_SCALE)),
            player: Player { velocity: 0.0 },
        }
    }
}

pub fn update_player(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &mut Transform), Without<Pipe>>,
    pipe_transform_query: Query<&Transform, With<Pipe>>,
    mut pipe_entity_query: Query<Entity, With<Pipe>>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    game_manager: Res<GameManager>,
    sprite_manager: Res<SpriteManager>,
    audio_manager: Res<AudioManager>,
) {
    if let Ok((mut player, mut player_transform)) = player_query.get_single_mut() {
        update_player_movement(
            &mut commands,
            keys,
            &mut player,
            &mut player_transform,
            time,
            &audio_manager,
        );

        if player_lost(&player_transform, pipe_transform_query, &game_manager) {
            commands.spawn(AudioPlayer::new(audio_manager.smack_sound.clone()));
            player_transform.translation = Vec3::ZERO;
            player.velocity = 0.0;
            for entity in pipe_entity_query.iter_mut() {
                commands.entity(entity).despawn();
            }

            spawn_pipes(
                &mut commands,
                game_manager.window_dimensions.x,
                &sprite_manager.pipe_sprite,
            );
        }
    }
}

fn update_player_movement(
    commands: &mut Commands,
    keys: Res<ButtonInput<KeyCode>>,
    player: &mut Mut<Player>,
    player_transform: &mut Mut<Transform>,
    time: Res<Time>,
    audio_manager: &Res<AudioManager>,
) {
    if keys.just_pressed(FLAP_KEY) {
        player.velocity = FLAP_FORCE;
        commands.spawn(AudioPlayer::new(audio_manager.flap_sound.clone()));
    }

    player.velocity -= time.delta_secs() * GRAVITY_STRENGTH;
    player_transform.translation.y += player.velocity * time.delta_secs();

    player_transform.rotation = Quat::from_axis_angle(
        Vec3::Z,
        f32::clamp(player.velocity / ROTATION_RATIO, -90.0, 90.0).to_radians(),
    );
}

fn player_lost(
    player_transform: &Mut<Transform>,
    pipe_transform_query: Query<&Transform, With<Pipe>>,
    game_manager: &Res<GameManager>,
) -> bool {
    player_hit_pipe(player_transform, pipe_transform_query)
        || player_hit_screen(player_transform, game_manager)
}

fn player_hit_pipe(
    player_transform: &Mut<Transform>,
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

fn player_hit_screen(player_transform: &Mut<Transform>, game_manager: &Res<GameManager>) -> bool {
    player_transform.translation.y <= -game_manager.window_dimensions.y / 2.0
        || player_transform.translation.y >= game_manager.window_dimensions.y / 2.0
}

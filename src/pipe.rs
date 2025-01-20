use crate::assets::{SpriteManager, PIPE_SPRITE_Z, SPRITE_SCALE};
use crate::game::WindowManager;
use crate::player::PlayerState;
use bevy::app::{App, FixedUpdate, Plugin};
use bevy::asset::Handle;
use bevy::image::Image;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{
    in_state, Bundle, Commands, Component, Entity, IntoSystemConfigs, Query, Res, Transform,
    Window, With,
};
use bevy::sprite::Sprite;
use bevy::time::Time;
use bevy::window::PrimaryWindow;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

const PIPE_AMOUNT: i32 = 4;
pub(crate) const PIPE_WIDTH: f32 = 18.0;
pub(crate) const PIPE_HEIGHT: f32 = 144.0;
pub(crate) const PIPE_GAP_SIZE: f32 = 15.0;
const PIPE_VERTICAL_OFFSET: f32 = 30.0;
const PIPE_SPACING: f32 = 60.0;
const PIPE_SPEED: f32 = 150.0;

pub(crate) const PIPE_HALF_WIDTH: f32 = (PIPE_WIDTH * SPRITE_SCALE) / 2.0;
pub(crate) const PIPE_HALF_HEIGHT: f32 = (PIPE_HEIGHT * SPRITE_SCALE) / 2.0;
pub(crate) const SCORE_ZONE_WIDTH: f32 = PIPE_WIDTH * 0.05;
const PIPE_VERTICAL_CENTER: f32 = (PIPE_HEIGHT / 2.0 + PIPE_GAP_SIZE) * SPRITE_SCALE;

const DIRECTION_UP: f32 = 1.0;
const DIRECTION_DOWN: f32 = -1.0;

#[derive(Component)]
pub(crate) struct Pipe {
    direction: f32,
}

#[derive(Component)]
pub(crate) struct ScoreZone;

#[derive(Bundle)]
pub(crate) struct PipeBundle {
    pipe: Pipe,
    sprite: Sprite,
    transform: Transform,
}

impl PipeBundle {
    pub(crate) fn new(translation: Vec2, direction: f32, pipe_image: &Handle<Image>) -> PipeBundle {
        PipeBundle {
            sprite: Sprite {
                image: pipe_image.clone(),
                ..Default::default()
            },
            transform: Transform::from_translation(translation.extend(PIPE_SPRITE_Z))
                .with_scale(Vec3::new(SPRITE_SCALE, SPRITE_SCALE * -direction, 1.0)),
            pipe: Pipe { direction },
        }
    }
}

pub(crate) struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            update_pipe_transform.run_if(in_state(PlayerState::Flapping)),
        );
    }
}

pub(crate) fn update_pipe_transform(
    mut commands: Commands,
    mut pipe_query: Query<(Entity, &mut Pipe, &mut Transform)>,
    game_manager: Res<WindowManager>,
    time: Res<Time>,
) {
    let mut pipes_to_reset = Vec::new();

    for (_, pipe, transform) in pipe_query.iter() {
        if transform.translation.x + PIPE_WIDTH * SPRITE_SCALE / 2.0
            < -game_manager.window_dimensions.x / 2.0
        {
            pipes_to_reset.push(pipe.direction);
        }
    }

    if !pipes_to_reset.is_empty() {
        let mut rand = thread_rng();
        let y_offset = generate_pipe_offset(&mut rand);

        for (entity, pipe, mut transform) in pipe_query.iter_mut() {
            transform.translation.x -= time.delta_secs() * PIPE_SPEED;

            if transform.translation.x + PIPE_WIDTH * SPRITE_SCALE / 2.0
                < -game_manager.window_dimensions.x / 2.0
            {
                transform.translation.x += PIPE_AMOUNT as f32 * PIPE_SPACING * SPRITE_SCALE;
                transform.translation.y = PIPE_VERTICAL_CENTER * pipe.direction + y_offset;
                if pipe.direction == DIRECTION_DOWN {
                    commands.entity(entity).insert(ScoreZone);
                }
            }
        }
    } else {
        for (_, _, mut transform) in pipe_query.iter_mut() {
            transform.translation.x -= time.delta_secs() * PIPE_SPEED;
        }
    }
}

pub(crate) fn spawn_pipes(
    mut commands: Commands,
    sprite_manager: Res<SpriteManager>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for i in 0..PIPE_AMOUNT {
        let y_offset = generate_pipe_offset(&mut thread_rng());
        if let Ok(window) = window_query.get_single() {
            let x_pos = window.width() / 2.0 + (PIPE_SPACING * SPRITE_SCALE * i as f32);
            commands.spawn(PipeBundle::new(
                Vec2::X * x_pos + Vec2::Y * (PIPE_VERTICAL_CENTER + y_offset),
                DIRECTION_UP,
                &sprite_manager.pipe_sprite.clone(),
            ));
            commands.spawn((
                PipeBundle::new(
                    Vec2::X * x_pos + Vec2::Y * (-PIPE_VERTICAL_CENTER + y_offset),
                    DIRECTION_DOWN,
                    &sprite_manager.pipe_sprite.clone(),
                ),
                ScoreZone,
            ));
        }
    }
}

pub(crate) fn despawn_pipes(mut commands: Commands, pipe_query: Query<Entity, With<Pipe>>) {
    for entity in pipe_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub(crate) fn generate_pipe_offset(rand: &mut ThreadRng) -> f32 {
    rand.gen_range(-PIPE_VERTICAL_OFFSET..PIPE_VERTICAL_OFFSET) * SPRITE_SCALE
}

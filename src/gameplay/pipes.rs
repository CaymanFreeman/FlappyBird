use super::*;
use bevy::window::PrimaryWindow;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

#[derive(Component)]
pub(crate) struct Pipe {
    direction: f32,
}

#[derive(Bundle)]
struct PipeBundle {
    pipe: Pipe,
    sprite: Sprite,
    transform: Transform,
}

impl PipeBundle {
    fn new(translation: Vec2, direction: f32, pipe_image: &Handle<Image>) -> PipeBundle {
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

pub(crate) fn update_pipe_transforms(
    mut pipe_query: Query<(&mut Pipe, &mut Transform)>,
    time: Res<Time>,
) {
    let mut pipes_to_reset = Vec::new();

    for (pipe, transform) in pipe_query.iter() {
        if transform.translation.x + PIPE_WIDTH_SCALED / 2.0 < WINDOW_MIN_X {
            pipes_to_reset.push(pipe.direction);
        }
    }

    if !pipes_to_reset.is_empty() {
        let mut rand = thread_rng();
        let y_offset = generate_pipe_offset(&mut rand);

        for (pipe, mut transform) in pipe_query.iter_mut() {
            transform.translation.x -= time.delta_secs() * PIPE_SPEED;

            if transform.translation.x + PIPE_WIDTH_SCALED / 2.0 < WINDOW_MIN_X {
                transform.translation.x += PIPE_AMOUNT as f32 * PIPE_SPACING * SPRITE_SCALE;
                transform.translation.y = PIPE_VERTICAL_CENTER * pipe.direction + y_offset;
            }
        }
    } else {
        for (_, mut transform) in pipe_query.iter_mut() {
            transform.translation.x -= time.delta_secs() * PIPE_SPEED;
        }
    }
}

pub(crate) fn spawn_pipes(
    mut commands: Commands,
    sprites: Res<SpriteAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for i in 0..PIPE_AMOUNT {
        let y_offset = generate_pipe_offset(&mut thread_rng());
        if let Ok(window) = window_query.get_single() {
            let x_pos = window.width() / 2.0 + (PIPE_SPACING * SPRITE_SCALE * i as f32);
            commands.spawn(PipeBundle::new(
                Vec2::X * x_pos + Vec2::Y * (PIPE_VERTICAL_CENTER + y_offset),
                PIPE_DIRECTION_UP,
                &sprites.pipe_sprite.clone(),
            ));
            commands.spawn(PipeBundle::new(
                Vec2::X * x_pos + Vec2::Y * (-PIPE_VERTICAL_CENTER + y_offset),
                PIPE_DIRECTION_DOWN,
                &sprites.pipe_sprite.clone(),
            ));
        }
    }
}

pub(crate) fn despawn_pipes(mut commands: Commands, pipe_query: Query<Entity, With<Pipe>>) {
    for entity in pipe_query.iter() {
        commands.entity(entity).despawn();
    }
}

fn generate_pipe_offset(rand: &mut ThreadRng) -> f32 {
    rand.gen_range(-PIPE_VERTICAL_OFFSET..PIPE_VERTICAL_OFFSET) * SPRITE_SCALE
}

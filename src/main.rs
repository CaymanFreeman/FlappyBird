use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Flappy Bird"),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(512.0, 512.0).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_systems(Startup, setup_game)
        .add_systems(Update, (update_player, update_pipes))
        .run();
}

const SPRITE_SCALE: f32 = 4.0;

const PLAYER_WIDTH: f32 = 12.0;
const PLAYER_HEIGHT: f32 = 8.0;
const PLAYER_COLLISION_RATIO: f32 = 0.3;

const FLAP_FORCE: f32 = 500.0;
const GRAVITY_STRENGTH: f32 = 2000.0;
const ROTATION_RATIO: f32 = 17.0;

#[derive(Component)]
struct Player {
    pub velocity: f32,
}

fn update_player(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &mut Transform), Without<Pipe>>,
    pipe_transform_query: Query<&Transform, With<Pipe>>,
    mut pipe_entity_query: Query<Entity, With<Pipe>>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    game_manager: Res<GameManager>,
) {
    if let Ok((mut player, mut player_transform)) = player_query.get_single_mut() {
        if keys.just_pressed(KeyCode::Space) {
            player.velocity = FLAP_FORCE;
        }

        player.velocity -= time.delta_secs() * GRAVITY_STRENGTH;
        player_transform.translation.y += player.velocity * time.delta_secs();

        player_transform.rotation = Quat::from_axis_angle(
            Vec3::Z,
            f32::clamp(player.velocity / ROTATION_RATIO, -90.0, 90.0).to_radians(),
        );

        let mut player_lost = false;

        if player_transform.translation.y <= -game_manager.window_dimensions.y / 2.0 {
            player_lost = true;
        }

        if !player_lost {
            let player_radius =
                (PLAYER_WIDTH.min(PLAYER_HEIGHT) * SPRITE_SCALE) * PLAYER_COLLISION_RATIO;
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
                    player_lost = true;
                    break;
                }
            }
        }

        if player_lost {
            player_transform.translation = Vec3::ZERO;
            player.velocity = 0.;
            for entity in pipe_entity_query.iter_mut() {
                commands.entity(entity).despawn();
            }
            let mut rand = thread_rng();
            spawn_pipes(
                &mut commands,
                &mut rand,
                game_manager.window_dimensions.x,
                &game_manager.pipe_image,
            );
        }
    }
}

const PIPE_AMOUNT: i32 = 5;
const PIPE_WIDTH: f32 = 18.0;
const PIPE_HEIGHT: f32 = 144.0;
const PIPE_VERTICAL_OFFSET: f32 = 30.0;
const PIPE_GAP_SIZE: f32 = 15.0;
const PIPE_SPACING: f32 = 60.0;
const PIPE_SPEED: f32 = 150.0;
const PIPE_CENTER: f32 = (PIPE_HEIGHT / 2.0 + PIPE_GAP_SIZE) * SPRITE_SCALE;

#[derive(Resource)]
struct GameManager {
    pipe_image: Handle<Image>,
    window_dimensions: Vec2,
}

#[derive(Component)]
struct Pipe {
    direction: f32,
}

fn update_pipes(
    mut pipe_query: Query<(&mut Pipe, &mut Transform)>,
    game_manager: Res<GameManager>,
    time: Res<Time>,
) {
    let mut pipes_to_reset = Vec::new();

    for (pipe, transform) in pipe_query.iter() {
        if transform.translation.x + PIPE_WIDTH * SPRITE_SCALE / 2.0
            < -game_manager.window_dimensions.x / 2.0
        {
            pipes_to_reset.push(pipe.direction);
        }
    }

    if !pipes_to_reset.is_empty() {
        let mut rand = thread_rng();
        let y_offset = generate_pipe_offset(&mut rand);

        for (pipe, mut transform) in pipe_query.iter_mut() {
            transform.translation.x -= time.delta_secs() * PIPE_SPEED;

            if transform.translation.x + PIPE_WIDTH * SPRITE_SCALE / 2.0
                < -game_manager.window_dimensions.x / 2.0
            {
                transform.translation.x += PIPE_AMOUNT as f32 * PIPE_SPACING * SPRITE_SCALE;
                transform.translation.y = PIPE_CENTER * pipe.direction + y_offset;
            }
        }
    } else {
        for (_pipe, mut transform) in pipe_query.iter_mut() {
            transform.translation.x -= time.delta_secs() * PIPE_SPEED;
        }
    }
}

fn spawn_pipes(
    commands: &mut Commands,
    rand: &mut ThreadRng,
    window_width: f32,
    pipe_image: &Handle<Image>,
) {
    for i in 0..PIPE_AMOUNT {
        let y_offset = generate_pipe_offset(rand);
        let x_pos = window_width / 2.0 + (PIPE_SPACING * SPRITE_SCALE * i as f32);
        spawn_pipe(
            commands,
            Vec3::X * x_pos + Vec3::Y * (PIPE_CENTER + y_offset),
            1.0,
            pipe_image,
        );
        spawn_pipe(
            commands,
            Vec3::X * x_pos + Vec3::Y * (-PIPE_CENTER + y_offset),
            -1.0,
            pipe_image,
        );
    }
}

fn spawn_pipe(
    commands: &mut Commands,
    translation: Vec3,
    direction: f32,
    pipe_image: &Handle<Image>,
) {
    commands.spawn((
        Sprite {
            image: pipe_image.clone(),
            ..Default::default()
        },
        Transform::from_translation(translation).with_scale(Vec3::new(
            SPRITE_SCALE,
            SPRITE_SCALE * -direction,
            SPRITE_SCALE,
        )),
        Pipe { direction },
    ));
}

fn generate_pipe_offset(rand: &mut ThreadRng) -> f32 {
    rand.gen_range(-PIPE_VERTICAL_OFFSET..PIPE_VERTICAL_OFFSET) * SPRITE_SCALE
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let pipe_image = asset_server.load("pipe.png");
    let window = window_query.get_single().expect("Window should exist");

    commands.insert_resource(GameManager {
        pipe_image: pipe_image.clone(),
        window_dimensions: Vec2::new(window.width(), window.height()),
    });

    commands.insert_resource(ClearColor(Color::srgb(0.5, 0.7, 0.8)));
    commands.spawn(Camera2d::default());
    commands.spawn((
        Sprite {
            image: asset_server.load("bird.png"),
            ..Default::default()
        },
        Transform::IDENTITY.with_scale(Vec3::splat(SPRITE_SCALE)),
        Player { velocity: 0.0 },
    ));

    let mut rand = thread_rng();
    spawn_pipes(&mut commands, &mut rand, window.width(), &pipe_image)
}

use crate::pipe::spawn_pipes;
use crate::player::Player;
use bevy::asset::{AssetServer, Handle};
use bevy::color::Color;
use bevy::image::Image;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Camera2d, ClearColor, Commands, Query, Res, Resource, Transform, With};
use bevy::sprite::Sprite;
use bevy::window::{PrimaryWindow, Window};
use rand::thread_rng;

pub const GAME_NAME: &str = "Flappy Bird";
pub const GAME_PIXEL_WIDTH: f32 = 512.0;
pub const GAME_PIXEL_HEIGHT: f32 = 512.0;

pub const SPRITE_SCALE: f32 = 4.0;
const PIPE_IMAGE: &str = "pipe.png";
const PLAYER_IMAGE: &str = "bird.png";

#[derive(Resource)]
pub struct GameManager {
    pub pipe_image: Handle<Image>,
    pub window_dimensions: Vec2,
}

pub fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let pipe_image = asset_server.load(PIPE_IMAGE);
    let window = window_query.get_single().expect("Window should exist");

    commands.insert_resource(GameManager {
        pipe_image: pipe_image.clone(),
        window_dimensions: Vec2::new(window.width(), window.height()),
    });

    commands.insert_resource(ClearColor(Color::srgb(0.5, 0.7, 0.8)));
    commands.spawn(Camera2d::default());
    commands.spawn((
        Sprite {
            image: asset_server.load(PLAYER_IMAGE),
            ..Default::default()
        },
        Transform::IDENTITY.with_scale(Vec3::splat(SPRITE_SCALE)),
        Player { velocity: 0.0 },
    ));

    let mut rand = thread_rng();
    spawn_pipes(&mut commands, &mut rand, window.width(), &pipe_image)
}

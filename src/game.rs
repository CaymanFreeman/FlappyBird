use crate::pipe::{spawn_pipes, update_pipes};
use crate::player::{update_player, PlayerBundle};
use bevy::app::{App, Plugin, PluginGroup, Startup, Update};
use bevy::asset::{AssetServer, Handle};
use bevy::color::Color;
use bevy::image::Image;
use bevy::math::Vec2;
use bevy::prelude::{
    Camera2d, ClearColor, Commands, ImagePlugin, IntoSystemConfigs, Query, Res, Resource,
    WindowPlugin, With,
};
use bevy::window::{MonitorSelection, PrimaryWindow, Window, WindowPosition};
use bevy::DefaultPlugins;
use rand::thread_rng;

pub const SPRITE_SCALE: f32 = 4.0;
const PIPE_IMAGE: &str = "pipe.png";
const PLAYER_IMAGE: &str = "bird.png";
const BACKGROUND_COLOR: [f32; 3] = [0.502, 0.702, 0.8]; // #80b3cc

const WINDOW_PIXEL_WIDTH: f32 = 512.0;
const WINDOW_PIXEL_HEIGHT: f32 = 512.0;

#[derive(Resource)]
pub struct GameManager {
    pub pipe_image: Handle<Image>,
    pub window_dimensions: Vec2,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Flappy Bird"),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(WINDOW_PIXEL_WIDTH, WINDOW_PIXEL_HEIGHT).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        );
        app.add_systems(Startup, setup_game);
        app.add_systems(Update, (update_pipes, update_player).chain());
    }
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

    commands.insert_resource(ClearColor(Color::srgb_from_array(BACKGROUND_COLOR)));
    commands.spawn(Camera2d::default());
    commands.spawn(PlayerBundle::new(asset_server.load(PLAYER_IMAGE)));

    let mut rand = thread_rng();
    spawn_pipes(&mut commands, &mut rand, window.width(), &pipe_image)
}

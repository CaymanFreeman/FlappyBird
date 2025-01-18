use crate::assets::{setup_audio_manager, setup_sprite_manager, SpriteManager};
use crate::pipe::{spawn_pipes, update_pipes};
use crate::player::{update_player, PlayerBundle};
use bevy::app::{App, Plugin, PluginGroup, PreStartup, Startup, Update};
use bevy::math::Vec2;
use bevy::prelude::{
    Camera2d, Commands, ImagePlugin, IntoSystemConfigs, Query, Res, Resource, WindowPlugin, With,
};
use bevy::window::{MonitorSelection, PrimaryWindow, Window, WindowPosition};
use bevy::DefaultPlugins;

const WINDOW_PIXEL_WIDTH: f32 = 512.0;
const WINDOW_PIXEL_HEIGHT: f32 = 512.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::new(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(WINDOW_PIXEL_WIDTH, WINDOW_PIXEL_HEIGHT).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        );
        app.add_systems(
            PreStartup,
            (
                setup_game_manager,
                setup_sprite_manager,
                setup_audio_manager,
            ),
        );
        app.add_systems(Startup, setup_game);
        app.add_systems(Update, (update_pipes, update_player).chain());
    }
}

#[derive(Resource)]
pub struct GameManager {
    pub window_dimensions: Vec2,
}

fn setup_game(
    mut commands: Commands,
    sprite_manager: Res<SpriteManager>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().expect("Window should exist");

    commands.spawn(Camera2d::default());
    commands.spawn(PlayerBundle::new(&sprite_manager.player_sprite));

    spawn_pipes(&mut commands, window.width(), &sprite_manager.pipe_sprite)
}

pub fn setup_game_manager(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().expect("Window should exist");
    commands.insert_resource(GameManager {
        window_dimensions: Vec2::new(window.width(), window.height()),
    });
}

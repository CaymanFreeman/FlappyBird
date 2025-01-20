use crate::assets::GameAssetsPlugin;
use crate::menu::{
    despawn_main_menu, despawn_retry_menu, despawn_score_display, spawn_main_menu,
    spawn_retry_menu, spawn_score_display, MenuPlugin,
};
use crate::pipe::{despawn_pipes, spawn_pipes, PipePlugin};
use crate::player::{despawn_player, spawn_player, PlayerPlugin, PlayerState};
use bevy::app::{App, Plugin, PluginGroup};
use bevy::math::Vec2;
use bevy::prelude::{
    AppExtStates, Camera2d, Commands, ImagePlugin, IntoSystemConfigs, NextState, OnTransition,
    Query, ResMut, Resource, States, WindowPlugin, With,
};
use bevy::window::{MonitorSelection, PrimaryWindow, Window, WindowPosition};
use bevy::DefaultPlugins;
use std::cmp::PartialEq;

const WINDOW_PIXEL_WIDTH: f32 = 512.0;
const WINDOW_PIXEL_HEIGHT: f32 = 512.0;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum GameState {
    #[default]
    Loading,
    MainMenu,
    Playing,
    RetryMenu,
}

#[derive(Resource)]
pub(crate) struct WindowManager {
    pub window_dimensions: Vec2,
}

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

        app.init_state::<GameState>();
        app.add_systems(
            OnTransition {
                exited: GameState::Loading,
                entered: GameState::MainMenu,
            },
            (spawn_main_menu, spawn_camera),
        );
        app.add_systems(
            OnTransition {
                exited: GameState::MainMenu,
                entered: GameState::Playing,
            },
            (
                despawn_main_menu,
                spawn_score_display,
                spawn_player,
                spawn_pipes,
                |mut next_player_state: ResMut<NextState<PlayerState>>| {
                    next_player_state.set(PlayerState::WaitingToStart);
                },
            ),
        );
        app.add_systems(
            OnTransition {
                exited: GameState::Playing,
                entered: GameState::RetryMenu,
            },
            spawn_retry_menu,
        );
        app.add_systems(
            OnTransition {
                exited: GameState::RetryMenu,
                entered: GameState::Playing,
            },
            (
                (
                    despawn_retry_menu,
                    despawn_score_display,
                    despawn_player,
                    despawn_pipes,
                ),
                (spawn_score_display, spawn_player, spawn_pipes),
                |mut next_player_state: ResMut<NextState<PlayerState>>| {
                    next_player_state.set(PlayerState::WaitingToStart);
                },
            )
                .chain(),
        );
        app.add_systems(
            OnTransition {
                exited: GameState::RetryMenu,
                entered: GameState::MainMenu,
            },
            (
                (
                    despawn_retry_menu,
                    despawn_score_display,
                    despawn_player,
                    despawn_pipes,
                ),
                spawn_main_menu,
            )
                .chain(),
        );
        app.add_plugins((PlayerPlugin, PipePlugin, MenuPlugin, GameAssetsPlugin));
    }
}

pub(crate) fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

pub(crate) fn setup_game_manager(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = window_query.get_single() {
        commands.insert_resource(WindowManager {
            window_dimensions: Vec2::new(window.width(), window.height()),
        });
    }
}

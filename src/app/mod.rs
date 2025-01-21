use crate::assets::*;
use crate::gameplay::*;
use crate::ui::*;
use bevy::prelude::*;

mod app_state;
mod window;

// Re-exports
pub(crate) use app_state::*;
pub(crate) use window::*;

// Window
const WINDOW_PIXEL_WIDTH: f32 = 512.0;
const WINDOW_PIXEL_HEIGHT: f32 = 512.0;

pub struct AppPlugin;

impl Plugin for AppPlugin {
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
        )
        .init_state::<AppState>()
        .add_systems(Startup, insert_window_info)
        .add_systems(
            PostStartup,
            |mut next_app_state: ResMut<NextState<AppState>>| {
                next_app_state.set(AppState::MainMenu);
            },
        )
        .add_systems(
            OnTransition {
                exited: AppState::Loading,
                entered: AppState::MainMenu,
            },
            (spawn_main_menu, spawn_camera),
        )
        .add_systems(
            OnTransition {
                exited: AppState::MainMenu,
                entered: AppState::Playing,
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
        )
        .add_systems(
            OnTransition {
                exited: AppState::Playing,
                entered: AppState::RetryMenu,
            },
            spawn_retry_menu,
        )
        .add_systems(
            OnTransition {
                exited: AppState::RetryMenu,
                entered: AppState::Playing,
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
        )
        .add_systems(
            OnTransition {
                exited: AppState::RetryMenu,
                entered: AppState::MainMenu,
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
        )
        .add_plugins((GameplayPlugin, GameAssetsPlugin, GameUiPlugin));
    }
}

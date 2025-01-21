use super::*;
use bevy::asset::AssetMetaCheck;

mod app_state;

// Re-exports
pub(crate) use app_state::*;

// Window
pub(crate) const WINDOW_WIDTH_PX: f32 = 512.0;
pub(crate) const WINDOW_HEIGHT_PX: f32 = 512.0;
pub(crate) const WINDOW_MIN_X: f32 = -WINDOW_WIDTH_PX / 2.0;
pub(crate) const WINDOW_MAX_Y: f32 = WINDOW_HEIGHT_PX / 2.0;
pub(crate) const WINDOW_MIN_Y: f32 = -WINDOW_HEIGHT_PX / 2.0;
const WINDOW_NAME: &str = "Flappy Bird";
const CANVAS_ID: &str = "#app";

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: WINDOW_NAME.to_string(),
                        canvas: Some(CANVAS_ID.into()),
                        resolution: Vec2::new(WINDOW_WIDTH_PX, WINDOW_HEIGHT_PX).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..Default::default()
                }),
        )
        .init_state::<AppState>()
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

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

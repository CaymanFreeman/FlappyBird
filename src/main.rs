use bevy::app::{App, PluginGroup, Startup, Update};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::Vec2;
use bevy::prelude::ImagePlugin;
use bevy::window::{MonitorSelection, Window, WindowPlugin, WindowPosition};
use bevy::DefaultPlugins;
use flappy_bird;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from(flappy_bird::GAME_NAME),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(
                            flappy_bird::GAME_PIXEL_WIDTH,
                            flappy_bird::GAME_PIXEL_HEIGHT,
                        )
                        .into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_systems(Startup, flappy_bird::setup_game)
        .add_systems(
            Update,
            (flappy_bird::update_player, flappy_bird::update_pipes),
        )
        .run();
}

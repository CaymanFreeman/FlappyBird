use bevy::app::App;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use flappy_bird;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(flappy_bird::AppPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin {
            wait_duration: Duration::from_secs(1),
            filter: Some(vec![FrameTimeDiagnosticsPlugin::FPS]),
            ..Default::default()
        })
        .run();
}

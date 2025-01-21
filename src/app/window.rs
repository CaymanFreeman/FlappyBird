use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Resource)]
pub(crate) struct WindowInfo {
    pub(crate) window_dimensions: Vec2,
}

pub(crate) fn insert_window_info(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = window_query.get_single() {
        commands.insert_resource(WindowInfo {
            window_dimensions: Vec2::new(window.width(), window.height()),
        });
    }
}

pub(crate) fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

use crate::assets::AudioManager;
use crate::game::{despawn_music, GameState, MENU_BUTTON};
use bevy::app::{App, Plugin, PostStartup, Update};
use bevy::audio::{AudioPlayer, PlaybackSettings};
use bevy::input::ButtonInput;
use bevy::prelude::{Commands, KeyCode, NextState, OnEnter, OnExit, Res, ResMut, State};

pub(crate) struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu);
        app.add_systems(OnExit(GameState::Menu), despawn_music);
        app.add_systems(Update, handle_menu_toggle);
        app.add_systems(
            PostStartup,
            |mut next_game_state: ResMut<NextState<GameState>>| {
                next_game_state.set(GameState::Menu);
            },
        );
    }
}

pub(crate) fn setup_menu(mut commands: Commands, audio_manager: Res<AudioManager>) {
    commands.spawn((
        AudioPlayer::new(audio_manager.music.clone()),
        PlaybackSettings::LOOP,
    ));
}

pub(crate) fn handle_menu_toggle(
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(MENU_BUTTON) {
        match game_state.get() {
            GameState::Menu => next_state.set(GameState::Playing),
            GameState::Playing => next_state.set(GameState::Menu),
            GameState::Loading => (),
        }
    }
}

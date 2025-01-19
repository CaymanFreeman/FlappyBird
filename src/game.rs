use crate::assets::{AudioManager, GameAssetsPlugin, SpriteManager, SWOOSH_SOUND_VOLUME};
use crate::menu::{MenuPlugin, MenuSystems};
use crate::pipe::{spawn_pipes, Pipe, PipePlugin};
use crate::player::{Player, PlayerBundle, PlayerPlugin, PlayerState, Score};
use bevy::app::{App, Plugin, PluginGroup, Startup};
use bevy::audio::{AudioPlayer, PlaybackSettings, Volume};
use bevy::math::Vec2;
use bevy::prelude::{
    AppExtStates, Camera2d, Commands, Entity, ImagePlugin, NextState, OnEnter, Or, Query, Res,
    ResMut, Resource, States, WindowPlugin, With,
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

        app.add_systems(
            Startup,
            (|mut commands: Commands| {
                commands.spawn(Camera2d::default());
            },),
        );

        app.init_state::<GameState>();
        app.add_systems(OnEnter(GameState::Playing), (setup_game, despawn_music));

        app.add_plugins((PlayerPlugin, PipePlugin, MenuPlugin, GameAssetsPlugin));
    }
}

pub(crate) fn despawn_music(
    mut commands: Commands,
    despawn_query: Query<Entity, With<AudioPlayer>>,
) {
    for entity in despawn_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub(crate) fn setup_game(
    mut commands: Commands,
    sprite_manager: Res<SpriteManager>,
    audio_manager: Res<AudioManager>,
    menu_systems: Res<MenuSystems>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
    mut score: ResMut<Score>,
) {
    score.0 = 0;
    commands.run_system(menu_systems.update_score_system_id);
    next_player_state.set(PlayerState::WaitingToStart);
    commands.spawn((
        AudioPlayer::new(audio_manager.swoosh_sound.clone()),
        PlaybackSettings {
            volume: Volume::new(SWOOSH_SOUND_VOLUME),
            ..Default::default()
        },
    ));
    commands.spawn(PlayerBundle::new(&sprite_manager.player_sprite));
    if let Ok(window) = window_query.get_single() {
        spawn_pipes(&mut commands, window.width(), &sprite_manager.pipe_sprite)
    }
}

pub(crate) fn despawn_player_and_pipes(
    mut commands: Commands,
    despawn_query: Query<Entity, Or<(With<Player>, With<Pipe>)>>,
) {
    for entity in despawn_query.iter() {
        commands.entity(entity).despawn();
    }
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

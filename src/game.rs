use crate::assets::{setup_audio_manager, setup_sprite_manager, AudioManager, SpriteManager};
use crate::pipe::{spawn_pipes, update_pipe_transform, Pipe};
use crate::player::{
    handle_fall_animation, handle_player_collision, handle_player_input, update_player_transform,
    Player, PlayerBundle, PlayerState, FLAP_FORCE, FLAP_KEY,
};
use bevy::app::{App, FixedUpdate, Plugin, PluginGroup, PreStartup, Update};
use bevy::audio::AudioPlayer;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{
    in_state, AppExtStates, Camera2d, Commands, Component, Entity, ImagePlugin, IntoSystemConfigs,
    KeyCode, NextState, OnEnter, OnExit, Or, Query, Res, ResMut, Resource, State, States, Time,
    Timer, TimerMode, WindowPlugin, With, Without,
};
use bevy::window::{MonitorSelection, PrimaryWindow, Window, WindowPosition};
use bevy::DefaultPlugins;
use std::cmp::PartialEq;
use std::time::Duration;

const WINDOW_PIXEL_WIDTH: f32 = 512.0;
const WINDOW_PIXEL_HEIGHT: f32 = 512.0;

const MENU_BUTTON: KeyCode = KeyCode::Escape;

pub const FALL_SOUND_DELAY: f32 = 0.5;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
}

#[derive(Resource)]
pub struct GameManager {
    pub window_dimensions: Vec2,
}

#[derive(Component)]
pub struct FallDelayTimer(Timer);

impl FallDelayTimer {
    pub fn new() -> FallDelayTimer {
        FallDelayTimer(Timer::new(
            Duration::from_secs_f32(FALL_SOUND_DELAY),
            TimerMode::Once,
        ))
    }
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
            PreStartup,
            (
                setup_game_manager,
                setup_sprite_manager,
                setup_audio_manager,
                |mut commands: Commands| {
                    commands.spawn(Camera2d::default());
                },
            ),
        );

        app.add_systems(
            Update,
            update_fall_sound_delay_timer
                .run_if(in_state(GameState::Playing))
                .run_if(in_state(PlayerState::WaitingToFall)),
        );

        app.init_state::<GameState>();
        app.add_systems(Update, handle_menu_toggle);
        app.add_systems(OnEnter(GameState::Playing), setup_game);
        app.add_systems(OnExit(GameState::Playing), cleanup_game);

        app.init_state::<PlayerState>();
        app.add_systems(
            Update,
            handle_frozen_toggle
                .run_if(in_state(GameState::Playing))
                .run_if(in_state(PlayerState::WaitingToStart)),
        );
        app.add_systems(
            Update,
            handle_fall_animation
                .run_if(in_state(GameState::Playing))
                .run_if(in_state(PlayerState::Falling)),
        );
        app.add_systems(
            Update,
            handle_player_input
                .run_if(in_state(GameState::Playing))
                .run_if(in_state(PlayerState::Flapping)),
        );
        app.add_systems(
            FixedUpdate,
            (
                update_player_transform
                    .run_if(in_state(GameState::Playing))
                    .run_if(in_state(PlayerState::Flapping)),
                update_pipe_transform
                    .run_if(in_state(GameState::Playing))
                    .run_if(in_state(PlayerState::Flapping)),
                handle_player_collision
                    .run_if(in_state(GameState::Playing))
                    .run_if(in_state(PlayerState::Flapping)),
            )
                .chain(),
        );
    }
}

pub fn update_fall_sound_delay_timer(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FallDelayTimer)>,
    audio_manager: Res<AudioManager>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<PlayerState>>,
) {
    if let Ok((entity, mut delay_timer)) = query.get_single_mut() {
        if delay_timer.0.tick(time.delta()).just_finished() {
            next_state.set(PlayerState::Falling);
            commands.spawn(AudioPlayer::new(audio_manager.fall_sound.clone()));
            commands.entity(entity).despawn();
        }
    }
}

fn setup_game(
    mut commands: Commands,
    sprite_manager: Res<SpriteManager>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut next_state: ResMut<NextState<PlayerState>>,
) {
    next_state.set(PlayerState::WaitingToStart);
    commands.spawn(PlayerBundle::new(&sprite_manager.player_sprite));
    if let Ok(window) = window_query.get_single() {
        spawn_pipes(&mut commands, window.width(), &sprite_manager.pipe_sprite)
    }
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

fn handle_menu_toggle(
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(MENU_BUTTON) {
        match game_state.get() {
            GameState::Menu => next_state.set(GameState::Playing),
            GameState::Playing => next_state.set(GameState::Menu),
        }
    }
}

fn handle_frozen_toggle(
    mut commands: Commands,
    mut player_query: Query<&mut Player, Without<Pipe>>,
    player_state: Res<State<PlayerState>>,
    mut next_state: ResMut<NextState<PlayerState>>,
    keys: Res<ButtonInput<KeyCode>>,
    audio_manager: Res<AudioManager>,
) {
    if keys.just_pressed(FLAP_KEY) {
        if let PlayerState::WaitingToStart = player_state.get() {
            next_state.set(PlayerState::Flapping);
            commands.spawn(AudioPlayer::new(audio_manager.flap_sound.clone()));
            if let Ok(mut player) = player_query.get_single_mut() {
                player.velocity = FLAP_FORCE;
            }
        }
    }
}

fn cleanup_game(mut commands: Commands, query: Query<Entity, Or<(With<Player>, With<Pipe>)>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

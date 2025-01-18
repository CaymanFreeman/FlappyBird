use crate::assets::{setup_audio_manager, setup_sprite_manager, SpriteManager};
use crate::pipe::{spawn_pipes, update_pipe_transform, Pipe};
use crate::player::{
    handle_player_collision, handle_player_input, update_player_transform, Player, PlayerBundle,
};
use bevy::app::{App, FixedUpdate, Plugin, PluginGroup, PreStartup, Update};
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{
    in_state, AppExtStates, Camera2d, Commands, Entity, ImagePlugin, IntoSystemConfigs, KeyCode,
    NextState, OnEnter, OnExit, Or, Query, Res, ResMut, Resource, State, States, WindowPlugin,
    With,
};
use bevy::window::{MonitorSelection, PrimaryWindow, Window, WindowPosition};
use bevy::DefaultPlugins;

const WINDOW_PIXEL_WIDTH: f32 = 512.0;
const WINDOW_PIXEL_HEIGHT: f32 = 512.0;

const MENU_BUTTON: KeyCode = KeyCode::Escape;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
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

        app.init_state::<GameState>();
        app.add_systems(Update, check_menu_toggle);
        app.add_systems(OnEnter(GameState::Playing), setup_game);
        app.add_systems(OnExit(GameState::Playing), cleanup_game);

        app.add_systems(
            Update,
            handle_player_input.run_if(in_state(GameState::Playing)),
        );
        app.add_systems(
            FixedUpdate,
            (
                update_player_transform.run_if(in_state(GameState::Playing)),
                update_pipe_transform.run_if(in_state(GameState::Playing)),
                handle_player_collision.run_if(in_state(GameState::Playing)),
            )
                .chain(),
        );
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

fn check_menu_toggle(
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

fn cleanup_game(mut commands: Commands, query: Query<Entity, Or<(With<Player>, With<Pipe>)>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

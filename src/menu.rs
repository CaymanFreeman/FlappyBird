use crate::assets::{AudioManager, FontManager, MUSIC_VOLUME};
use crate::game::{despawn_player_and_pipes, GameState};
use bevy::app::{App, Plugin, PostStartup, Update};
use bevy::asset::Handle;
use bevy::audio::{AudioPlayer, PlaybackMode, PlaybackSettings, Volume};
use bevy::color::Color;
use bevy::hierarchy::{BuildChildren, ChildBuild, DespawnRecursiveExt};
use bevy::prelude::{
    in_state, AlignItems, Button, Changed, Commands, Component, Display, Entity, IntoSystemConfigs,
    NextState, OnEnter, OnExit, Query, Res, ResMut, Text, Val, With,
};
use bevy::text::{Font, FontSmoothing, TextColor, TextFont};
use bevy::ui::{
    BackgroundColor, BorderColor, BorderRadius, FlexDirection, Interaction, JustifyContent, Node,
    PositionType, UiRect,
};

pub(crate) const TITLE_TEXT: &str = "FlappyBird";
pub(crate) const PLAY_BUTTON_TEXT: &str = "Play";
pub(crate) const RETRY_BUTTON_TEXT: &str = "Retry";
pub(crate) const MAIN_MENU_BUTTON_TEXT: &str = "Menu";

pub(crate) const BUTTON_BACKGROUND_COLOR: [f32; 3] = [0.984, 0.949, 0.212]; // #fbf236
pub(crate) const BUTTON_BACKGROUND_COLOR_HOVER: [f32; 3] = [0.902, 0.867, 0.2]; // #e6dd33
pub(crate) const BUTTON_BORDER_COLOR: [f32; 3] = [0.0, 0.0, 0.0]; // #000000
pub(crate) const BUTTON_TEXT_COLOR: [f32; 3] = [0.0, 0.0, 0.0]; // #000000
pub(crate) const BUTTON_WIDTH_PX: f32 = 200.0;
pub(crate) const BUTTON_HEIGHT_PX: f32 = 70.0;
pub(crate) const BUTTON_BORDER_PX: f32 = 2.5;
pub(crate) const BUTTON_BORDER_RADIUS_PX: f32 = 10.0;
pub(crate) const BUTTON_FONT_SIZE_PX: f32 = 75.0;

pub(crate) const PLAY_BUTTON_OFFSET_PX: f32 = 50.0;

pub(crate) const TITLE_FONT_SIZE_PX: f32 = 75.0;
pub(crate) const TITLE_TOP_MARGIN_PX: f32 = 85.0;
pub(crate) const TITLE_OUTLINE_WIDTH_PX: f32 = 3.5;
pub(crate) const TITLE_COLOR: [f32; 3] = [1.0, 1.0, 1.0]; // #FFFFFF
pub(crate) const TITLE_OUTLINE_COLOR: [f32; 3] = [0.0, 0.0, 0.0]; // #000000

pub(crate) const RETRY_MENU_ROW_GAP_PX: f32 = 8.0;

pub(crate) struct MenuPlugin;

#[derive(Component)]
pub(crate) struct MainMenu;

#[derive(Component)]
pub(crate) struct PlayButton;

#[derive(Component)]
pub(crate) struct RetryButton;

#[derive(Component)]
pub(crate) struct MainMenuButton;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::MainMenu),
            (setup_main_menu, despawn_player_and_pipes),
        );
        app.add_systems(OnExit(GameState::MainMenu), despawn_main_menu);
        app.add_systems(
            Update,
            (
                pressed_play_button.run_if(in_state(GameState::MainMenu)),
                update_button_hover.run_if(in_state(GameState::MainMenu)),
            ),
        );
        app.add_systems(
            PostStartup,
            |mut next_game_state: ResMut<NextState<GameState>>| {
                next_game_state.set(GameState::MainMenu);
            },
        );
    }
}

pub(crate) fn setup_main_menu(
    mut commands: Commands,
    audio_manager: Res<AudioManager>,
    font_manager: Res<FontManager>,
) {
    commands.spawn((
        AudioPlayer::new(audio_manager.music.clone()),
        PlaybackSettings {
            volume: Volume::new(MUSIC_VOLUME),
            mode: PlaybackMode::Loop,
            ..Default::default()
        },
    ));
    spawn_main_menu(
        commands,
        &font_manager.title_font,
        &font_manager.button_font,
    );
}

pub(crate) fn spawn_main_menu(
    mut commands: Commands,
    title_font: &Handle<Font>,
    button_font: &Handle<Font>,
) {
    commands
        .spawn((
            Node {
                display: Display::Flex,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(TITLE_TOP_MARGIN_PX),
                    ..Default::default()
                },))
                .with_children(|parent| {
                    parent.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            top: Val::Px(TITLE_OUTLINE_WIDTH_PX),
                            ..Default::default()
                        },
                        Text::new(TITLE_TEXT),
                        TextFont {
                            font: title_font.clone(),
                            font_size: TITLE_FONT_SIZE_PX,
                            font_smoothing: FontSmoothing::None,
                        },
                        TextColor::from(Color::srgb_from_array(TITLE_OUTLINE_COLOR)),
                    ));
                    parent.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            top: Val::Px(-TITLE_OUTLINE_WIDTH_PX),
                            ..Default::default()
                        },
                        Text::new(TITLE_TEXT),
                        TextFont {
                            font: title_font.clone(),
                            font_size: TITLE_FONT_SIZE_PX,
                            font_smoothing: FontSmoothing::None,
                        },
                        TextColor::from(Color::srgb_from_array(TITLE_OUTLINE_COLOR)),
                    ));
                    parent.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(-TITLE_OUTLINE_WIDTH_PX),
                            ..Default::default()
                        },
                        Text::new(TITLE_TEXT),
                        TextFont {
                            font: title_font.clone(),
                            font_size: TITLE_FONT_SIZE_PX,
                            font_smoothing: FontSmoothing::None,
                        },
                        TextColor::from(Color::srgb_from_array(TITLE_OUTLINE_COLOR)),
                    ));
                    parent.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(TITLE_OUTLINE_WIDTH_PX),
                            ..Default::default()
                        },
                        Text::new(TITLE_TEXT),
                        TextFont {
                            font: title_font.clone(),
                            font_size: TITLE_FONT_SIZE_PX,
                            font_smoothing: FontSmoothing::None,
                        },
                        TextColor::from(Color::srgb_from_array(TITLE_OUTLINE_COLOR)),
                    ));
                    parent.spawn((
                        Text::new(TITLE_TEXT),
                        TextFont {
                            font: title_font.clone(),
                            font_size: TITLE_FONT_SIZE_PX,
                            font_smoothing: FontSmoothing::None,
                        },
                        TextColor::from(Color::srgb_from_array(TITLE_COLOR)),
                    ));
                });
            parent
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Percent(50.0),
                        left: Val::Percent(50.0),
                        width: Val::Px(BUTTON_WIDTH_PX),
                        height: Val::Px(BUTTON_HEIGHT_PX),
                        margin: UiRect {
                            left: Val::Px(-BUTTON_WIDTH_PX / 2.0),
                            top: Val::Px(-BUTTON_HEIGHT_PX / 2.0 + PLAY_BUTTON_OFFSET_PX),
                            ..Default::default()
                        },
                        border: UiRect::all(Val::Px(BUTTON_BORDER_PX)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderColor(Color::srgb_from_array(BUTTON_BORDER_COLOR)),
                    BorderRadius::all(Val::Px(BUTTON_BORDER_RADIUS_PX)),
                    BackgroundColor(Color::srgb_from_array(BUTTON_BACKGROUND_COLOR)),
                    Button,
                    PlayButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(PLAY_BUTTON_TEXT),
                        TextFont {
                            font: button_font.clone(),
                            font_size: BUTTON_FONT_SIZE_PX,
                            font_smoothing: FontSmoothing::None,
                        },
                        TextColor::from(Color::srgb_from_array(BUTTON_TEXT_COLOR)),
                    ));
                });
        });
}

pub(crate) fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>,
) {
    if let Ok(main_menu) = main_menu_query.get_single() {
        commands.entity(main_menu).despawn_recursive();
    }
}

pub(crate) fn update_button_hover(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match interaction {
            Interaction::Hovered => {
                background_color.0 = Color::srgb_from_array(BUTTON_BACKGROUND_COLOR_HOVER);
            }
            Interaction::None => {
                background_color.0 = Color::srgb_from_array(BUTTON_BACKGROUND_COLOR);
            }
            Interaction::Pressed => (),
        }
    }
}

pub(crate) fn pressed_play_button(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(interaction) = button_query.get_single_mut() {
        if let Interaction::Pressed = interaction {
            next_game_state.set(GameState::Playing)
        }
    }
}

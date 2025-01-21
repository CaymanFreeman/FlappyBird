use super::*;
use crate::app::AppState;
use crate::assets::{AudioAssets, FontAssets, MUSIC_VOLUME, SWOOSH_SOUND_VOLUME};
use bevy::audio::{PlaybackMode, Volume};
use bevy::text::FontSmoothing;

#[derive(Component)]
pub(crate) struct MainMenu;

#[derive(Component)]
pub(crate) struct PlayButton;

pub(crate) fn spawn_main_menu(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    audio: Res<AudioAssets>,
) {
    commands.spawn((
        AudioPlayer::new(audio.music.clone()),
        PlaybackSettings {
            volume: Volume::new(MUSIC_VOLUME),
            mode: PlaybackMode::Loop,
            ..Default::default()
        },
    ));
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
                .spawn(Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(TITLE_TOP_MARGIN_PX),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            top: Val::Px(TITLE_OUTLINE_WIDTH_PX),
                            ..Default::default()
                        },
                        Text::new(TITLE_TEXT),
                        TextFont {
                            font: fonts.title_font.clone(),
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
                            font: fonts.title_font.clone(),
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
                            font: fonts.title_font.clone(),
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
                            font: fonts.title_font.clone(),
                            font_size: TITLE_FONT_SIZE_PX,
                            font_smoothing: FontSmoothing::None,
                        },
                        TextColor::from(Color::srgb_from_array(TITLE_OUTLINE_COLOR)),
                    ));
                    parent.spawn((
                        Text::new(TITLE_TEXT),
                        TextFont {
                            font: fonts.title_font.clone(),
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
                    BackgroundColor(Color::srgb_from_array(BUTTON_COLOR)),
                    Button,
                    PlayButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(PLAY_BUTTON_TEXT),
                        TextFont {
                            font: fonts.button_font.clone(),
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
    audio_player_query: Query<Entity, With<AudioPlayer>>,
    audio: Res<AudioAssets>,
) {
    if let Ok(main_menu) = main_menu_query.get_single() {
        commands.entity(main_menu).despawn_recursive();
    }
    for entity in audio_player_query.iter() {
        commands.entity(entity).despawn();
    }
    commands.spawn((
        AudioPlayer::new(audio.swoosh_sound.clone()),
        PlaybackSettings {
            volume: Volume::new(SWOOSH_SOUND_VOLUME),
            ..Default::default()
        },
    ));
}

pub(crate) fn handle_play_button(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok(interaction) = button_query.get_single_mut() {
        if let Interaction::Pressed = interaction {
            next_app_state.set(AppState::Playing)
        }
    }
}

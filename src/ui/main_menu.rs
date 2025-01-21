use super::*;
use crate::app::AppState;
use crate::assets::{AudioAssets, FontAssets, MUSIC_VOLUME, SWOOSH_SOUND_VOLUME};
use bevy::audio::{PlaybackMode, Volume};

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
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((Node {
                    position_type: PositionType::Absolute,
                    justify_self: JustifySelf::Center,
                    top: Val::Px(TITLE_TOP_MARGIN_PX),
                    ..Default::default()
                },))
                .with_children(|parent| {
                    spawn_outlined_text(
                        parent,
                        TITLE_TEXT,
                        fonts.title_font.clone(),
                        TITLE_FONT_SIZE_PX,
                        TITLE_COLOR,
                        TITLE_OUTLINE_COLOR,
                        TITLE_OUTLINE_WIDTH_PX,
                    );
                });
            spawn_button(
                parent,
                BUTTON_WIDTH_PX,
                BUTTON_HEIGHT_PX,
                PLAY_BUTTON_TEXT,
                fonts.button_font.clone(),
                PlayButton,
            );
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

use super::*;
use crate::app::AppState;
use crate::assets::FontAssets;
use bevy::text::FontSmoothing;

#[derive(Component)]
pub(crate) struct RetryMenu;

#[derive(Component)]
pub(crate) struct RetryButton;

#[derive(Component)]
pub(crate) struct MainMenuButton;

pub(crate) fn spawn_retry_menu(mut commands: Commands, fonts: Res<FontAssets>) {
    commands
        .spawn((
            Node {
                display: Display::Flex,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                row_gap: Val::Px(RETRY_MENU_ROW_GAP_PX),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            RetryMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(BUTTON_WIDTH_PX),
                        height: Val::Px(BUTTON_HEIGHT_PX),
                        border: UiRect::all(Val::Px(BUTTON_BORDER_PX)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderColor(Color::srgb_from_array(BUTTON_BORDER_COLOR)),
                    BorderRadius::all(Val::Px(BUTTON_BORDER_RADIUS_PX)),
                    BackgroundColor(Color::srgb_from_array(BUTTON_COLOR)),
                    Button,
                    RetryButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(RETRY_BUTTON_TEXT),
                        TextFont {
                            font: fonts.button_font.clone(),
                            font_size: BUTTON_FONT_SIZE_PX,
                            font_smoothing: FontSmoothing::None,
                        },
                        TextColor::from(Color::srgb_from_array(BUTTON_TEXT_COLOR)),
                    ));
                });
            parent
                .spawn((
                    Node {
                        width: Val::Px(BUTTON_WIDTH_PX),
                        height: Val::Px(BUTTON_HEIGHT_PX),
                        border: UiRect::all(Val::Px(BUTTON_BORDER_PX)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderColor(Color::srgb_from_array(BUTTON_BORDER_COLOR)),
                    BorderRadius::all(Val::Px(BUTTON_BORDER_RADIUS_PX)),
                    BackgroundColor(Color::srgb_from_array(BUTTON_COLOR)),
                    Button,
                    MainMenuButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(MAIN_MENU_BUTTON_TEXT),
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

pub(crate) fn despawn_retry_menu(
    mut commands: Commands,
    retry_menu_query: Query<Entity, With<RetryMenu>>,
) {
    if let Ok(retry_menu) = retry_menu_query.get_single() {
        commands.entity(retry_menu).despawn_recursive();
    }
}

pub(crate) fn handle_retry_button(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<RetryButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok(interaction) = button_query.get_single_mut() {
        if let Interaction::Pressed = interaction {
            next_app_state.set(AppState::Playing)
        }
    }
}

pub(crate) fn handle_main_menu_button(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<MainMenuButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok(interaction) = button_query.get_single_mut() {
        if let Interaction::Pressed = interaction {
            next_app_state.set(AppState::MainMenu)
        }
    }
}

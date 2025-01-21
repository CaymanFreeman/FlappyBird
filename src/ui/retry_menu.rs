use super::*;
use crate::app::AppState;
use crate::assets::FontAssets;

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
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(RETRY_MENU_ROW_GAP_PX),
                ..Default::default()
            },
            RetryMenu,
        ))
        .with_children(|parent| {
            spawn_button(
                parent,
                BUTTON_WIDTH_PX,
                BUTTON_HEIGHT_PX,
                RETRY_BUTTON_TEXT,
                fonts.button_font.clone(),
                RetryButton,
            );
            spawn_button(
                parent,
                BUTTON_WIDTH_PX,
                BUTTON_HEIGHT_PX,
                MAIN_MENU_BUTTON_TEXT,
                fonts.button_font.clone(),
                MainMenuButton,
            );
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

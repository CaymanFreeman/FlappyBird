use bevy::prelude::*;
use bevy::text::FontSmoothing;

mod main_menu;
mod retry_menu;
mod score;

// Re-exports
use crate::assets::SPRITE_SCALE;
use crate::gameplay::{PIPE_SPACING, PIPE_SPEED};
pub(crate) use main_menu::*;
pub(crate) use retry_menu::*;
pub(crate) use score::*;

// Outlined Text
const OUTLINE_OFFSETS: [[f32; 2]; 4] = [[0.0, 1.0], [0.0, -1.0], [1.0, 0.0], [-1.0, 0.0]];

// General Buttons
const BUTTON_COLOR: [f32; 3] = [0.984, 0.949, 0.212]; // #fbf236
const BUTTON_HOVER_COLOR: [f32; 3] = [0.902, 0.867, 0.2]; // #e6dd33
const BUTTON_BORDER_COLOR: [f32; 3] = [0.0, 0.0, 0.0]; // #000000
const BUTTON_TEXT_COLOR: [f32; 3] = [0.0, 0.0, 0.0]; // #000000
const BUTTON_WIDTH_PX: f32 = 200.0;
const BUTTON_HEIGHT_PX: f32 = 70.0;
const BUTTON_BORDER_PX: f32 = 2.5;
const BUTTON_BORDER_RADIUS_PX: f32 = 10.0;
const BUTTON_FONT_SIZE_PX: f32 = 75.0;

// Main Menu
const TITLE_TEXT: &str = "FlappyBird";
const TITLE_FONT_SIZE_PX: f32 = 75.0;
const TITLE_TOP_MARGIN_PX: f32 = 85.0;
const TITLE_OUTLINE_WIDTH_PX: f32 = 3.5;
const TITLE_COLOR: [f32; 3] = [1.0, 1.0, 1.0]; // #FFFFFF
const TITLE_OUTLINE_COLOR: [f32; 3] = [0.0, 0.0, 0.0]; // #000000

const PLAY_BUTTON_TEXT: &str = "Play";

// Retry Menu
const RETRY_MENU_ROW_GAP_PX: f32 = 8.0;
const RETRY_BUTTON_TEXT: &str = "Retry";
const MAIN_MENU_BUTTON_TEXT: &str = "Menu";

// Score Display
const SCORE_DISPLAY_FONT_SIZE_PX: f32 = 50.0;
const SCORE_DISPLAY_TOP_MARGIN_PX: f32 = 30.0;
const SCORE_DISPLAY_OUTLINE_WIDTH_PX: f32 = 2.5;
const SCORE_DISPLAY_COLOR: [f32; 3] = [1.0, 1.0, 1.0]; // #FFFFFF
const SCORE_DISPLAY_OUTLINE_COLOR: [f32; 3] = [0.0, 0.0, 0.0]; // #000000

const SCORE_TIMER_INTERVAL: f32 = (PIPE_SPACING * SPRITE_SCALE) / PIPE_SPEED;

pub(crate) struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_play_button,
                handle_retry_button,
                handle_main_menu_button,
                handle_button_hover,
                handle_score_timer,
            ),
        );
    }
}

pub(crate) fn spawn_outlined_text(
    parent: &mut ChildBuilder,
    text: &str,
    font: Handle<Font>,
    font_size: f32,
    text_color: [f32; 3],
    outline_color: [f32; 3],
    outline_width: f32,
) {
    for offset in OUTLINE_OFFSETS.iter() {
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(offset[0] * outline_width),
                left: Val::Px(offset[1] * outline_width),
                ..Default::default()
            },
            Text::new(text),
            TextFont {
                font: font.clone(),
                font_size,
                font_smoothing: FontSmoothing::None,
            },
            TextColor::from(Color::srgb_from_array(outline_color)),
        ));
    }
    parent.spawn((
        Text::new(text),
        TextFont {
            font,
            font_size,
            font_smoothing: FontSmoothing::None,
        },
        TextColor::from(Color::srgb_from_array(text_color)),
    ));
}

pub(crate) fn spawn_button(
    parent: &mut ChildBuilder,
    text: &str,
    font: Handle<Font>,
    button_component: impl Bundle,
) {
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
            button_component,
        ))
        .with_children(|parent| spawn_button_text(parent, text, font));
}

pub(crate) fn spawn_button_text(parent: &mut ChildBuilder, text: &str, font: Handle<Font>) {
    parent.spawn((
        Text::new(text),
        TextFont {
            font,
            font_size: BUTTON_FONT_SIZE_PX,
            font_smoothing: FontSmoothing::None,
        },
        TextColor::from(Color::srgb_from_array(BUTTON_TEXT_COLOR)),
    ));
}

fn handle_button_hover(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match interaction {
            Interaction::Hovered => {
                background_color.0 = Color::srgb_from_array(BUTTON_HOVER_COLOR);
            }
            Interaction::None => {
                background_color.0 = Color::srgb_from_array(BUTTON_COLOR);
            }
            Interaction::Pressed => (),
        }
    }
}

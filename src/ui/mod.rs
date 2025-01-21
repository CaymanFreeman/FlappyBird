use bevy::prelude::*;

mod main_menu;
mod retry_menu;
mod score;

// Re-exports
use crate::assets::SPRITE_SCALE;
use crate::gameplay::{PIPE_SPACING, PIPE_SPEED};
pub(crate) use main_menu::*;
pub(crate) use retry_menu::*;
pub(crate) use score::*;

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
const PLAY_BUTTON_OFFSET_PX: f32 = 50.0;

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

mod game;
mod pipe;
mod player;

pub use crate::game::setup_game;
pub use crate::game::{GAME_NAME, GAME_PIXEL_HEIGHT, GAME_PIXEL_WIDTH};
pub use crate::pipe::update_pipes;
pub use crate::player::update_player;

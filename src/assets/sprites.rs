use super::*;

#[derive(Resource)]
pub(crate) struct SpriteAssets {
    pub(crate) pipe_sprite: Handle<Image>,
    pub(crate) player_sprite: Handle<Image>,
}

pub(crate) fn insert_sprite_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ClearColor(Color::srgb_from_array(GAME_BACKGROUND_COLOR)));
    commands.insert_resource(SpriteAssets {
        pipe_sprite: asset_server.load(PIPE_SPRITE),
        player_sprite: asset_server.load(PLAYER_SPRITE),
    });
}

use super::*;

#[derive(Resource)]
pub(crate) struct FontAssets {
    pub(crate) button_font: Handle<Font>,
    pub(crate) title_font: Handle<Font>,
}

pub(crate) fn insert_font_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(FontAssets {
        button_font: asset_server.load(BUTTON_FONT),
        title_font: asset_server.load(TITLE_FONT),
    });
}

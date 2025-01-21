use super::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum AppState {
    #[default]
    Loading,
    MainMenu,
    Playing,
    RetryMenu,
}

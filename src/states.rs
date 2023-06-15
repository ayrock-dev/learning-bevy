use bevy::prelude::*;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum AppState {
  #[default]
  LoadingGameplay,
  InGame,
  MainMenu,
}

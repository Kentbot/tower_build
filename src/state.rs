use crate::prelude::*;

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_state::<AppState>()
      .init_state::<GameState>()
      .init_state::<MenuState>()
      ;
  }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum AppState {
  #[default]
  Loading,
  Ready
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum MenuState {
  #[default]
  MainMenu,
  InGame
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum GameState {
  #[default]
  Build,
}
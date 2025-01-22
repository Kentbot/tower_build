mod actions;
mod assets;
mod camera;
mod cursor;
mod domino_grid;

use actions::*;
use assets::*;
use camera::*;
use cursor::*;
use domino_grid::*;
use transform::world_to_transform;

use crate::prelude::*;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins((
        ActionsPlugin,
        CursorPlugin,
        TempPlugin,
        DominoesPlugin,
      ))
      .add_systems(Startup, (
        (
          init_resources,
          init_assets
        ).chain(),
        init_camera,
      ))
      .add_systems(Update, (
        check_assets_loading.run_if(in_state(AppState::Loading)),
      ))
      ;
  }
}

fn init_camera(
  mut commands: Commands,
) {
  commands.spawn((
    Camera2d,
    MainCamera,
    world_to_transform(8., 6., 1000.)
  ));
}

fn init_resources(
  mut commands: Commands,
) {
  commands.insert_resource(LoadedAssets::default());
  commands.insert_resource(AssetsLoading::default());
}

// ~-~-~-~-~-~-~-~-~-~-~ TEMP STUFF ~-~-~-~-~-~-~-~-~-~-~
pub struct TempPlugin;

impl Plugin for TempPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(AppState::Ready), go_to_in_game)
      ;
  }
}

fn go_to_in_game(
  mut game_state: ResMut<NextState<MenuState>>
) {
  game_state.set(MenuState::InGame)
}
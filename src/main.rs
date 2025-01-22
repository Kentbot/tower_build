mod state;
mod systems;
mod helpers;

mod prelude {
  pub use bevy::prelude::*;

  pub use crate::helpers::*;
  pub use crate::state::*;
  pub use crate::systems::*;
}

use prelude::*;

fn main() {
  App::new()
    .add_plugins(
      DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(get_window_plugin())
    )
    .add_plugins(StatesPlugin)
    .add_plugins(SystemsPlugin)
    .run();
}

pub const WINDOW_WIDTH: f32 = 640. * 2.;
pub const WINDOW_HEIGHT: f32 = 360. * 2.;

pub fn get_window_plugin() -> WindowPlugin {
  WindowPlugin {
    primary_window: Some(Window {
      title: "Domino Prototype".into(),
      name: Some("protodominoes.app".into()),
      resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
      // Tells wasm to resize the window according to the available canvas
      fit_canvas_to_parent: true,
      // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
      prevent_default_event_handling: false,
      enabled_buttons: bevy::window::EnabledButtons {
        maximize: false,
        ..Default::default()
      },
      // mode: bevy::window::WindowMode::BorderlessFullscreen(
      //   MonitorSelection::Primary
      // ),
      ..default()
    }),
    ..default()
  }
}
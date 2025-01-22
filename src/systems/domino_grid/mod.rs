pub mod components;
pub mod events;
mod systems;

use bevy::input::common_conditions::{input_just_pressed, input_toggle_active};
use systems::*;
use events::*;

use crate::prelude::*;

pub struct DominoesPlugin;

impl Plugin for DominoesPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins((
        DominoGridEventsPlugin,
      ))
      .add_systems(OnEnter(MenuState::InGame), (
        systems::init,
        init_grid,
      ))
      .add_systems(Update, (
        on_domino_spawn,
        on_draggable_spawn,
        handle_dragging,
        on_grid_square_spawn,
        handle_horizontal_domino_placed,
        handle_vertical_domino_placed,
        debug_draw_placement_areas.run_if(input_toggle_active(false, KeyCode::Escape)),
        handle_drag_end.run_if(on_event::<DragEndEvent>),
        (
          reset,
          init,
          init_grid,
        ).chain().run_if(input_just_pressed(KeyCode::Backspace))
      ))
      ;
  }
}
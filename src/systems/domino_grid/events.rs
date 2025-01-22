use crate::prelude::*;

pub struct DominoGridEventsPlugin;

impl Plugin for DominoGridEventsPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<DominoPlacementEvent>()
      .add_event::<DragEndEvent>()
      ;
  }
}

/**
 The event when anything is no longer being dragged.
 This can happen anywhere, and so it should handle the case of the domino
 being dropped both inside and outside of a droppable area
 */
#[derive(Event)]
pub struct DragEndEvent {
  pub draggable: Entity,
}

/**
 This is when the domino is placed in the grid's droppable area.
 The domino should then be added to the grid, and sprites and droppable areas should be
 updated.
 */
#[derive(Event)]
pub enum DominoPlacementEvent {
  Horizontal {
    left_cell_col: i32,
    row: i32,
  },
  Vertical {
    bottom_cell_row: i32,
    column: i32,
  }
}
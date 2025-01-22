use bevy::window::PrimaryWindow;

use crate::prelude::*;

use super::camera::*;

#[derive(Resource, Default)]
pub struct CursorWorldCoords(pub Vec2);

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<CursorWorldCoords>()
      .add_systems(Update, (
        update_cursor_world_pos,
      ))
      ;
  }
}

pub fn update_cursor_world_pos(
  mut mycoords: ResMut<CursorWorldCoords>,
  // query to get the window (so we can read the current cursor position)
  q_window: Query<&Window, With<PrimaryWindow>>,
  // query to get camera transform
  q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
  // get the camera info and transform
  // assuming there is exactly one main camera entity, so Query::single() is OK
  let (camera, camera_transform) = q_camera.single();

  // There is only one primary window, so we can similarly get it from the query:
  let window = q_window.single();

  // check if the cursor is inside the window and get its position
  // then, ask bevy to convert into world coordinates, and truncate to discard Z
  if let Some(cursor) = window.cursor_position() {
    if let Ok(world_coords) = camera.viewport_to_world_2d(camera_transform, cursor) {
      mycoords.0 = world_coords;
    }
  }
}
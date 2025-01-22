use crate::prelude::*;

pub const TILE_SIZE: f32 = 32.;

/**
  Transform tile world coordinates to actual screen pixels based
  on the overall tile size.
  
  Note that only x and y are scaled so
  that you have control over z values.
 */
pub fn world_to_transform(x: f32, y: f32, z: f32) -> Transform {
  Transform::from_xyz(
    x * TILE_SIZE,
    y * TILE_SIZE,
    z
  )
}
use crate::prelude::*;

#[derive(Component)]
pub struct Draggable;

#[derive(Component)]
pub struct IsDragging {
  pub start_location: Vec2,
}
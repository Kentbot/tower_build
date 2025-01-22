use bevy::math::bounding::Aabb2d;

use crate::prelude::*;

#[derive(Component)]
pub struct Domino {
  pub d_type: DominoType,
  pub horizontal: bool,
}

#[derive(Component, Clone, Copy)]
pub enum DominoType {
  Blue(DominoColor),
  Red(DominoColor),
  Green(DominoColor),
  Yellow(DominoColor),
}

impl DominoType {
  pub const ALL_TYPES: [Self; 16] = [
    DominoType::Blue(DominoColor::Blue),
    DominoType::Blue(DominoColor::Green),
    DominoType::Blue(DominoColor::Yellow),
    DominoType::Blue(DominoColor::Red),

    DominoType::Green(DominoColor::Green),
    DominoType::Green(DominoColor::Blue),
    DominoType::Green(DominoColor::Yellow),
    DominoType::Green(DominoColor::Red),

    DominoType::Red(DominoColor::Red),
    DominoType::Red(DominoColor::Blue),
    DominoType::Red(DominoColor::Green),
    DominoType::Red(DominoColor::Yellow),

    DominoType::Yellow(DominoColor::Yellow),
    DominoType::Yellow(DominoColor::Blue),
    DominoType::Yellow(DominoColor::Green),
    DominoType::Yellow(DominoColor::Red),
  ];
}

#[derive(Clone, Copy)]
pub enum DominoColor {
  Blue,
  Red,
  Green,
  Yellow,
}

#[derive(Component)]
pub struct GridSquare;

#[derive(Component)]
pub enum PlacementArea {
  Horizontal {
    left_cell_col: i32,
    row: i32
  },
  Vertical {
    bottom_cell_row: i32,
    column: i32
  }
}

impl PlacementArea {
  pub fn is_this_point_within(&self, point: Vec2) -> bool {
    match *self {
      PlacementArea::Horizontal { left_cell_col: left_cell, row } => {
        let center = Vec2::new((16 + (32 * left_cell)) as f32, 32. * row as f32);
        let aabb = Aabb2d::new(
          center,
          Vec2::splat(16.)
        );
        
        let x_within_bb = point.x < aabb.max.x && point.x > aabb.min.x;
        let y_within_bb = point.y < aabb.max.y && point.y > aabb.min.y;
        let point_within_bb = x_within_bb && y_within_bb;

        point_within_bb
      }
      PlacementArea::Vertical { bottom_cell_row: bottom_cell, column } => {
        let center = Vec2::new((32 * column) as f32, 16. + (32. * bottom_cell as f32));
        let aabb = Aabb2d::new(
          center,
          Vec2::splat(16.)
        );
        
        let x_within_bb = point.x < aabb.max.x && point.x > aabb.min.x;
        let y_within_bb = point.y < aabb.max.y && point.y > aabb.min.y;
        let point_within_bb = x_within_bb && y_within_bb;

        point_within_bb
      }
    }
  }

  pub fn transform(&self) -> Transform {
    match *self {
      PlacementArea::Horizontal { left_cell_col: left_cell, row } => {
        Transform::from_translation(Vec2::new((16 + (32 * left_cell)) as f32, 32. * row as f32).extend(1.))
      },
      PlacementArea::Vertical { bottom_cell_row: bottom_cell, column } => {
        Transform::from_translation(Vec2::new((32 * column) as f32, 16. + (32. * bottom_cell as f32)).extend(1.))
      }
    }
  }
}
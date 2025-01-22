use core::f32;

use crate::prelude::*;
use crate::systems::cursor::CursorWorldCoords;
use crate::systems::DominoControlAction;
use crate::systems::LoadedAssets;

use super::components::*;
use super::events::*;

use bevy::color::palettes::css::GREEN;
use bevy::color::palettes::css::RED;
use bevy::sprite::Anchor;
use dragging::Draggable;
use dragging::IsDragging;
use leafwing_input_manager::prelude::ActionState;
use transform::world_to_transform;

pub fn init(
  mut commands: Commands,
) {
  let spawn_area_y: f32 = 12.;
  for (index, domino) in DominoType::ALL_TYPES.iter().enumerate() {
    let x_offset = ((index % 8) * 3) as f32;
    let y_offset = ((index / 8) * 2) as f32;
    commands.spawn((
      Domino {
        d_type: domino.clone(),
        horizontal: true,
      },
      Draggable,
      world_to_transform(-4. + x_offset, spawn_area_y + y_offset, 1.),
    ));
  }

  commands.spawn((
    Node {
      left: Val::Px(0.),
      ..default()
    },
  )).with_children(|parent| {
    parent.spawn((
      Text::new("Press [ESC] to toggle (debug) hitboxes in the grid. [BACKSPACE] to reset the dominoes.
Press [R] to rotate dominoes when dragging.
Red hitboxes are for horizontal dominoes. Green hitboxes are for vertical dominoes"),
      Anchor::TopLeft,
    ));
  });
}

pub fn init_grid(
  mut commands: Commands,
) {
  let grid_width = 10;
  let grid_height = 10;
  for i in 0..grid_width {
    for j in 0..grid_height {
      let x_offset = 0.;
      let y_offset = 0.;
      commands.spawn((
        GridSquare,
        world_to_transform(i as f32 + x_offset, j as f32 + y_offset, 0.5),
      ));

      if i < grid_width - 1 {
        commands.spawn((
          PlacementArea::Horizontal { left_cell_col: i, row: j },
        ));
      }

      if j < grid_height - 1 {
        commands.spawn((
          PlacementArea::Vertical { bottom_cell_row: j, column: i },
        ));
      }
    }
  }
}

pub fn debug_draw_placement_areas(
  mut gizmos: Gizmos,
  placement_areas: Query<&PlacementArea>
) {
  for placement in placement_areas.iter() {
    match *placement {
      PlacementArea::Horizontal { .. } => {
        gizmos
          .rect_2d(
            Isometry2d::from_translation(placement.transform().translation.truncate()),
            Vec2::new(32., 32.),
            RED
          );
      },
      PlacementArea::Vertical { .. } => {
        gizmos
          .rect_2d(
            Isometry2d::from_translation(placement.transform().translation.truncate()),
            Vec2::new(32., 32.),
            GREEN
          );
      }
    }
  }
}

pub fn on_grid_square_spawn(
  mut commands: Commands,
  loaded_assets: Res<LoadedAssets>,
  squares: Query<Entity, Added<GridSquare>>,
) {
  for square in squares.iter() {
    commands.entity(square)
      .insert((
        loaded_assets.images.grid_square.get_sprite(),
      ));
  }
}

pub fn on_draggable_spawn(
  mut commands: Commands,
  draggables: Query<Entity, Added<Draggable>>,
) {
  for draggable in draggables.iter() {
    commands.entity(draggable)
      .observe(|
        trigger: Trigger<Pointer<Down>>,
        mut commands: Commands,
        draggable: Query<(Entity, &Transform), With<Draggable>>,
      | {
        if let Ok((entity, transform)) = draggable.get(trigger.entity()) {
          commands.entity(entity).insert(IsDragging {
            start_location: transform.translation.truncate()
          });
        }
      })
      .observe(|
        trigger: Trigger<Pointer<Drag>>,
        mut transforms: Query<&mut Transform, With<Draggable>>,
        mycoords: Res<CursorWorldCoords>,
      | {
        if let Ok(mut transform) = transforms.get_mut(trigger.entity()) {
          transform.translation.x = mycoords.0.x;
          transform.translation.y = mycoords.0.y;
        }
      })
      .observe(|
        trigger: Trigger<Pointer<DragEnd>>,
        mut drag_end_evw: EventWriter<DragEndEvent>,
        draggables: Query<Entity, With<Draggable>>
      | {
        if let Ok(draggable) = draggables.get(trigger.entity()) {
          drag_end_evw.send(DragEndEvent {
            draggable
          });
        }
      });
  }
}

pub fn handle_dragging(
  mut dominoes: Query<(&mut Transform, &mut Domino)>,
  q_dragging: Query<Entity, With<IsDragging>>,
  action_state: Res<ActionState<DominoControlAction>>,
) {
  for dragging in q_dragging.iter() {
    if let Ok((mut transform, mut domino)) = dominoes.get_mut(dragging) {
      if action_state.just_pressed(&DominoControlAction::Rotate) {
        transform.rotate_local(Quat::from_rotation_z(f32::consts::FRAC_PI_2));
        domino.horizontal = !domino.horizontal;
      }
    }
  }
}

pub fn on_domino_spawn(
  mut commands: Commands,
  loaded_assets: Res<LoadedAssets>,
  dominoes: Query<(Entity, &Domino), Added<Domino>>,
) {
  for (entity, domino) in dominoes.iter() {
    commands.entity(entity).insert(
      loaded_assets.images.dominoes.get_domino(&domino.d_type)
    );
  }
}

pub fn handle_drag_end(
  mut commands: Commands,
  mut drag_end_evr: EventReader<DragEndEvent>,
  mut placement_evw: EventWriter<DominoPlacementEvent>,
  placement_areas: Query<&PlacementArea>,
  mut domino_query: Query<(Entity, &mut Transform, &IsDragging, &Domino)>,
) {
  for event in drag_end_evr.read() {
    if let Ok((entity, mut transform, dragging_info, domino)) = domino_query.get_mut(event.draggable) {
      for placement_area in placement_areas.iter() {
        match *placement_area {
          PlacementArea::Horizontal { left_cell_col: left_cell, row } => {
            if !domino.horizontal { continue; }

            if placement_area.is_this_point_within(transform.translation.truncate()) {
              transform.translation = placement_area.transform().translation.xy().extend(transform.translation.z);
              placement_evw.send(DominoPlacementEvent::Horizontal { left_cell_col: left_cell, row });
              commands.entity(entity)
                .remove::<IsDragging>()
                .remove::<Draggable>();
              return;
            }
          },
          PlacementArea::Vertical { bottom_cell_row: bottom_cell, column } => {
            if domino.horizontal { continue; }

            if placement_area.is_this_point_within(transform.translation.truncate()) {
              transform.translation = placement_area.transform().translation.xy().extend(transform.translation.z);
              placement_evw.send(DominoPlacementEvent::Vertical { bottom_cell_row: bottom_cell, column });
              commands.entity(entity)
                .remove::<IsDragging>()
                .remove::<Draggable>();
              return;
            }
          }
        }
      }

      transform.translation = dragging_info.start_location.extend(1.);
      // Remove the IsDragging
      commands.entity(entity)
        .remove::<IsDragging>();
    }
  }
}

pub fn handle_horizontal_domino_placed(
  mut commands: Commands,
  mut domino_placed_evr: EventReader<DominoPlacementEvent>,
  placement_areas: Query<(Entity, &PlacementArea)>,
) {
  for event in domino_placed_evr.read() {
    // Identify the event
    match *event {
      // This system only cares about horizontal events
      DominoPlacementEvent::Horizontal { left_cell_col: placed_left_cell_col, row: placed_row } => {
        // Iterate over all placement areas
        for (entity, placement_area) in placement_areas.iter() {
          match placement_area {
            // For horizontal placement areas, remove all PA's that are -1,0,+1 from the
            // left cell, but only within this row
            &PlacementArea::Horizontal { left_cell_col: left_cell, row } => {
              let should_remove = row == placed_row && (
                left_cell == placed_left_cell_col ||
                left_cell == placed_left_cell_col - 1 ||
                left_cell == placed_left_cell_col + 1 
              );
              if should_remove {
                commands.entity(entity).despawn();
              }
            },
            // For vertical placement areas, remove all PA's that are 0,+1 from the
            // left cell, for this row and the row below
            &PlacementArea::Vertical { bottom_cell_row, column } => {
              let should_remove =
                (bottom_cell_row == placed_row && placed_left_cell_col == column) ||
                (bottom_cell_row == placed_row && placed_left_cell_col + 1 == column) ||
                (bottom_cell_row == placed_row - 1 && placed_left_cell_col == column) ||
                (bottom_cell_row == placed_row - 1 && placed_left_cell_col + 1 == column)
                ;
              if should_remove {
                commands.entity(entity).despawn();
              }
            },
          }
        }
      },
      _ => {},
    }
  }
}

pub fn handle_vertical_domino_placed(
  mut commands: Commands,
  mut domino_placed_evr: EventReader<DominoPlacementEvent>,
  placement_areas: Query<(Entity, &PlacementArea)>,
) {
  for event in domino_placed_evr.read() {
    // Identify the event
    match *event {
      // This system only cares about horizontal events
      DominoPlacementEvent::Vertical { column: placed_col, bottom_cell_row: placed_row_bottom_cell } => {
        // Iterate over all placement areas
        for (entity, placement_area) in placement_areas.iter() {
          match placement_area {
            // For vertical placement areas, remove all PA's that are -1,0,+1 from the
            // bottom cell, but only within this column
            &PlacementArea::Vertical { bottom_cell_row, column } => {
              let should_remove = column == placed_col && (
                bottom_cell_row == placed_row_bottom_cell ||
                bottom_cell_row == placed_row_bottom_cell - 1 ||
                bottom_cell_row == placed_row_bottom_cell + 1 
              );
              if should_remove {
                commands.entity(entity).despawn();
              }
            },
            // For vertical placement areas, remove all PA's that are 0,+1 from the
            // left cell, for this row and the row below
            &PlacementArea::Horizontal { left_cell_col, row } => {
              let should_remove =
                (row == placed_row_bottom_cell && placed_col == left_cell_col) ||
                (row == placed_row_bottom_cell && placed_col - 1 == left_cell_col) ||
                (row == placed_row_bottom_cell + 1 && placed_col == left_cell_col) ||
                (row == placed_row_bottom_cell + 1 && placed_col - 1 == left_cell_col)
                ;
              if should_remove {
                commands.entity(entity).despawn();
              }
            },
          }
        }
      },
      _ => {},
    }
  }
}

pub fn reset(
  mut commands: Commands,
  dominoes: Query<Entity, With<Domino>>,
  placement_areas: Query<Entity, With<PlacementArea>>,
) {
  for domino in dominoes.iter() {
    commands.entity(domino).despawn_recursive();
  }

  for placement_area in placement_areas.iter() {
    commands.entity(placement_area).despawn_recursive();
  }
}
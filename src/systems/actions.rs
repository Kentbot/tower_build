use leafwing_input_manager::{plugin::InputManagerPlugin, prelude::{ActionState, InputMap}, Actionlike};

use crate::prelude::*;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(InputManagerPlugin::<DominoControlAction>::default())
      .init_resource::<ActionState<DominoControlAction>>()
      .insert_resource(DominoControlAction::default_input_map())
      ;
  }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum DominoControlAction {
  Rotate,
}

impl DominoControlAction {
  pub fn default_input_map() -> InputMap<Self> {
    let mut input_map = InputMap::default();

    input_map.insert(Self::Rotate, KeyCode::KeyR);
    
    input_map
  }
}

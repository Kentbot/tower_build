use crate::prelude::*;

use super::components::DominoType;

#[derive(Default, Resource)]
pub struct AssetsLoading(pub Vec<UntypedHandle>);

#[derive(Default, Resource)]
pub struct LoadedAssets {
  pub images: LoadedImages,
}

#[derive(Default)]
pub struct LoadedImages {
  pub ground: ImageAsset,
  pub dominoes: DominoesAssetInfo,
  pub grid_square: ImageAsset,
}

#[derive(Default)]
pub struct ImageAsset {
  pub image: Handle<Image>,
  pub atlas: Option<Handle<TextureAtlasLayout>>,
}

#[derive(Default)]
pub struct DominoesAssetInfo {
  image_asset: ImageAsset,
}

struct DominoInfo {
  index: usize, flipped: bool
}

impl DominoInfo {
  pub fn flipped(index: usize) -> Self {
    Self {
      index, flipped: true
    }
  }

  pub fn unflipped(index: usize) -> Self {
    Self {
      index, flipped: false
    }
  }
}

impl DominoesAssetInfo {
  fn sprite_info(&self, domino: &DominoType) -> DominoInfo {
    use super::domino_grid::components::DominoColor::*;
    match domino {
      DominoType::Blue(color) => {
        match color {
          Blue => DominoInfo::unflipped(0),
          Green => DominoInfo::unflipped(5),
          Red => DominoInfo::unflipped(7),
          Yellow => DominoInfo::unflipped(8),
        }
      },

      DominoType::Red(color) => {
        match color {
          Red => DominoInfo::unflipped(3),
          Blue => DominoInfo::flipped(self.sprite_info(&DominoType::Blue(Red)).index),
          Green => DominoInfo::unflipped(4),
          Yellow => DominoInfo::unflipped(9),
        }
      },

      DominoType::Green(color) => {
        match color {
          Green => DominoInfo::unflipped(2),
          Yellow => DominoInfo::unflipped(6),
          Blue => DominoInfo::flipped(self.sprite_info(&DominoType::Blue(Green)).index),
          Red => DominoInfo::flipped(self.sprite_info(&DominoType::Red(Green)).index),
        }
      },

      DominoType::Yellow(color) => {
        match color {
          Yellow => DominoInfo::unflipped(1),
          Blue => DominoInfo::flipped(self.sprite_info(&DominoType::Blue(Yellow)).index),
          Green => DominoInfo::flipped(self.sprite_info(&DominoType::Green(Yellow)).index),
          Red => DominoInfo::flipped(self.sprite_info(&DominoType::Red(Yellow)).index),
        }
      },
    }
  }

  pub fn get_domino(&self, domino: &DominoType) -> Sprite {
    Sprite {
      image: self.image_asset.image.clone(),
      texture_atlas: if self.image_asset.atlas.is_some() {
        let atlas = self.image_asset.atlas.as_ref().unwrap();
        Some(TextureAtlas {
          index: self.sprite_info(&domino).index,
          layout: atlas.clone()
        })
      } else { unreachable!("Dominoes atlas should be set on inititialization") },
      flip_x: self.sprite_info(&domino).flipped,
      ..default()
    }
  }
}

impl ImageAsset {
  pub fn get_sprite(&self) -> Sprite {
    Sprite {
      image: self.image.clone(),
      ..default()
    }
  }

  /** More generic, for getting a sprite at any index of a loaded assets atlas */
  pub fn get_sprite_at(&self, atlas_index: usize) -> Sprite {
    Sprite {
      image: self.image.clone(),
      texture_atlas: if self.atlas.is_some() {
        let atlas = self.atlas.as_ref().unwrap();
        Some(TextureAtlas {
          index: atlas_index,
          layout: atlas.clone()
        })
      } else { None },
      ..default()
    }
  }
}

pub fn init_assets(
  asset_server: Res<AssetServer>,
  mut loading_assets: ResMut<AssetsLoading>,
  mut loaded_assets: ResMut<LoadedAssets>,
  mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
  let ground_image = asset_server.load::<Image>("images/ground.png");
  let dominoes_image = asset_server.load::<Image>("images/dominoes.png");
  let placement_area = asset_server.load::<Image>("images/tile_placement_area.png");

  let dominoes_layout = TextureAtlasLayout::from_grid(UVec2::new(64, 32), 4, 3, None, None);
  let dominoes_atlas = texture_atlas_layouts.add(dominoes_layout);

  loading_assets.0.append(&mut vec![
    ground_image.clone().untyped(),
    dominoes_image.clone().untyped(),
    placement_area.clone().untyped(),
  ]);

  loaded_assets.images.ground = ImageAsset {
    image: ground_image,
    atlas: None
  };

  loaded_assets.images.dominoes = DominoesAssetInfo {
    image_asset: ImageAsset {
      image: dominoes_image,
      atlas: Some(dominoes_atlas)
    }
  };

  loaded_assets.images.grid_square = ImageAsset {
    image: placement_area,
    atlas: None
  };
}

pub fn check_assets_loading(
  server: Res<AssetServer>,
  assets_loading: Res<AssetsLoading>,
  mut app_state: ResMut<NextState<AppState>>,
) {
  let mut any_unloaded = false;

  for asset in assets_loading.0.iter() {
    any_unloaded = !server.is_loaded(asset);

    if any_unloaded { break; }
  }

  let all_loaded = !any_unloaded;
  debug!("Finished loading assets!");
  if all_loaded {
    app_state.set(AppState::Ready);
  }
}
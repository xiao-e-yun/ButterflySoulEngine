use crate::modules::{
  enity::EnityStatus,
  render::{Texture},
};

impl EnityStatus {
  pub fn set_texture(&mut self, texture: Texture) {
    self.texture = texture
  }
}

impl GetDisplayItem for EnityStatus {
  fn texture(&self) -> Texture {
    self.texture
  }
}

use crate::{
  modules::enity::EnityStatus,
  utils::{
    vector::Vector,
    viewbox::{ViewBox, ViewBoxCache},
  },
};

impl ViewBox for EnityStatus {
  fn angle(&self) -> f32 {
    self.angle
  }
  fn size(&self) -> Vector {
    self.size
  }
  fn position(&self) -> Vector {
    self.position.get()
  }
  fn cache(&self) -> Option<&ViewBoxCache> {
    Some(&self.cache.0)
  }
}

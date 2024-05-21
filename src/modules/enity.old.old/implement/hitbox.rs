use crate::{
  modules::enity::EnityStatus,
  utils::{
    hitbox::{HitBox, HitBoxCache},
    vector::Vector,
  },
};

impl HitBox for EnityStatus {
  fn angle(&self) -> f32 {
    self.angle
  }
  fn size(&self) -> Vector {
    self.size
  }
  fn position(&self) -> Vector {
    self.get_position()
  }
  fn cache(&self) -> Option<&HitBoxCache> {
    Some(&self.cache.1)
  }
}

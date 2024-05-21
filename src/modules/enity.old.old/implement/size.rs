use crate::{modules::enity::EnityStatus, utils::vector::Vector};

impl EnityStatus {
  pub fn get_size(&self) -> Vector {
    self.size
  }
  pub fn set_size(&mut self, size: Vector) {
    self.size = size
  }
  pub fn scale_size(&mut self, scale: f32) {
    self.size = self.size * scale
  }
}

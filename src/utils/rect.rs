use std::ops::Add;

use serde::{Deserialize, Serialize};

use super::{hitbox::HitBox, vector::Vector, viewbox::ViewBox};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Rect {
  pub position: Vector,
  pub size: Vector,
  pub angle: f32,
}

impl Rect {
  pub fn new(position: Vector, size: Vector) -> Rect {
    Rect {
      size,
      position,
      angle: 0.0,
    }
  }
  pub fn new_with_angle(position: Vector, size: Vector, angle: f32) -> Rect {
    let mut rect = Self::new(position,size);
    rect.set_angle(angle);
    rect
  }
  pub fn set_angle(&mut self, angle: f32) {
    self.angle = angle
  }
}

impl Add<Vector> for Rect {
  type Output = Rect;
  fn add(mut self, rhs: Vector) -> Self::Output {
    self.position.0 += rhs.0;
    self.position.1 += rhs.1;
    self
  }
}

impl ViewBox for Rect {
  fn angle(&self) -> f32 {
    self.angle
  }

  fn size(&self) -> Vector {
    self.size
  }

  fn position(&self) -> Vector {
    self.position
  }
}

impl HitBox for Rect {
  fn angle(&self) -> f32 {
    self.angle
  }

  fn size(&self) -> Vector {
    self.size
  }

  fn position(&self) -> Vector {
    self.position
  }
}

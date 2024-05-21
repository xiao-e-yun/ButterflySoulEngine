#[derive(Debug, Clone, Copy)]
pub struct Bar {
  pub max: f32,
  pub current: f32,
}

impl Bar {
  pub fn new(max: f32) -> Bar {
    Bar { max, current: max }
  }
  pub fn new_with_current(max: f32,current: f32) -> Bar {
    Bar { max, current }
  }
  pub fn proportion(&self) -> f32 {
    self.current / self.max
  }
}
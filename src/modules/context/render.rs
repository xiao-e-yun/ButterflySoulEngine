use std::mem;

use serde::{Deserialize, Serialize};

use crate::utils::{rect::Rect, vector::Vector, viewbox::ViewBox};

///==================================================================
/// Traits
///==================================================================

pub trait Render {
  fn render(&self, frame: &mut RenderFrame);
}

///==================================================================
/// RenderFrame
///==================================================================
#[derive(Debug, Clone)]
pub struct RenderFrame {
  entities: Vec<(Rect, Texture)>,
  ui: Vec<(Rect, Texture)>,
  viewport: ViewPort,
}

impl RenderFrame {
  pub fn new() -> Self {
    RenderFrame {
      ui: vec![],
      entities: vec![],
      viewport: ViewPort::new(),
    }
  }
  pub fn with_capacity(capacity: usize) -> Self {
    RenderFrame {
      ui: Vec::with_capacity(capacity),
      entities: Vec::with_capacity(capacity),
      viewport: ViewPort::new(),
    }
  }
  pub fn extend(&self) -> Self {
    RenderFrame {
      ui: Vec::with_capacity(self.ui.capacity()),
      entities: Vec::with_capacity(self.entities.capacity()),
      viewport: self.viewport.clone(),
    }
  }
  pub fn append(&mut self, input: &Vec<(Rect, Texture)>) {
    self.entities.extend(input.iter().cloned())
  }
  pub fn push(&mut self, input: (Rect, Texture)) {
    self.entities.push(input)
  }
  pub fn append_ui(&mut self, input: &Vec<(Rect, Texture)>) {
    self.ui.extend(input.iter().cloned())
  }
  pub fn push_ui(&mut self, input: (Rect, Texture)) {
    self.ui.push(input)
  }
  pub fn get(&self) -> &Vec<(Rect, Texture)> {
    &self.entities
  }
  pub fn get_ui(&self) -> &Vec<(Rect, Texture)> {
    &self.ui
  }
  pub fn set_viewport(&mut self, viewport: ViewPort) {
    self.viewport = viewport
  }
  pub fn viewport(&self) -> ViewPort {
    self.viewport.clone()
  }
  pub fn flush(&mut self) -> Self {
    let frame = self.extend();
    mem::replace(self, frame)
  }
  pub fn sort(&mut self) {
    self
      .entities
      .sort_by(|a, b| a.0.position.1.partial_cmp(&b.0.position.1).unwrap())
  }
}

///==================================================================
/// Texture
///==================================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Texture {
  Color(String),
  Bitmap(String),
}

impl Default for Texture {
  fn default() -> Self {
    Texture::Color(String::from("#000000"))
  }
}

///==================================================================
/// ViewPort
///==================================================================
#[derive(Debug, Clone, Copy)]
pub struct ViewPort {
  position: Vector,
  size: Vector,
}

impl ViewPort {
  pub fn new() -> Self {
    ViewPort {
      position: Vector::ORIGIN,
      size: Vector::new(1000., 1000.),
    }
  }
  //left - bottom
  pub fn origin(&self) -> Vector {
    self.position() - self.size() * 0.5
  }
  pub fn set_position(&mut self, vector: Vector) {
    self.position = vector
  }
  pub fn set_size(&mut self, vector: Vector) {
    self.size = vector
  }
  pub fn map_to_viewport(&self, vector: Vector) -> Vector {
    vector * self.size.1 / 2. + self.position
  }
}

impl ViewBox for ViewPort {
  fn angle(&self) -> f32 {
    0.0
  }
  fn position(&self) -> Vector {
    self.position
  }
  fn size(&self) -> Vector {
    self.size
  }
}

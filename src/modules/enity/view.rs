use std::cell::Cell;

use indexmap::IndexMap;

use crate::{modules::context::render::Texture, utils::rect::Rect};

#[derive(Debug, Clone)]
pub struct EnityView {
  viewbox: Cell<Option<Rect>>,
  viewboxes: IndexMap<String, Vec<(Rect, Texture)>>,
  hitboxes: IndexMap<String, Vec<Rect>>,
}

impl EnityView {
  pub fn new(viewbox: Vec<(Rect, Texture)>, hitbox: Vec<Rect>) -> EnityView {
    let mut viewboxes = IndexMap::new();
    let mut hitboxes = IndexMap::new();

    if !viewbox.is_empty() {
      viewboxes.insert("base".to_string(), viewbox);
    };

    if !hitbox.is_empty() {
      hitboxes.insert("base".to_string(), hitbox);
    };

    let viewbox = Cell::default();
    EnityView {
      viewbox,
      viewboxes,
      hitboxes,
    }
  }

  //================================================================================
  //  Insert Part
  //================================================================================
  pub fn insert(
    &mut self,
    name: String,
    part: Vec<(Rect, Texture)>,
  ) -> Option<Vec<(Rect, Texture)>> {
    let result = self.viewboxes.insert(name, part);
    result
  }
  pub fn remove(&mut self, name: &str) -> Option<Vec<(Rect, Texture)>> {
    let result = self.viewboxes.swap_remove(name);
    result
  }
  pub fn mixin(&mut self, name: String, part: Vec<(Rect, Texture)>) {
    if let Some(other_part) = self.viewboxes.get_mut(&name) {
      //other_part.collision = part.collision //SKIP
      other_part.append(&mut part.clone());
    } else {
      self.insert(name, part);
    }
  }

  pub fn insert_hitbox(&mut self, name: String, part: Vec<Rect>) -> Option<Vec<Rect>> {
    let result = self.hitboxes.insert(name, part);
    result
  }
  pub fn remove_hitbox(&mut self, name: &str) -> Option<Vec<Rect>> {
    let result = self.hitboxes.swap_remove(name);
    result
  }
  pub fn mixin_hitbox(&mut self, name: String, part: Vec<Rect>) {
    if let Some(other_part) = self.hitboxes.get_mut(&name) {
      other_part.append(&mut part.clone());
    } else {
      self.insert_hitbox(name, part);
    }
  }
  //================================================================================
  // ViewBox
  //================================================================================
  pub fn viewboxes(&self) -> Vec<(Rect, Texture)> {
    self.viewboxes.values().flatten().cloned().collect()
  }
  //================================================================================
  // HitBox
  //================================================================================
  pub fn hitboxes(&self) -> Vec<Rect> {
    self.hitboxes.values().flatten().cloned().collect()
  }
}

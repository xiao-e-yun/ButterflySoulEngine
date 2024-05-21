use std::{
  collections::HashSet,
  fmt::Debug,
  hash::{Hash, Hasher},
};

use crate::utils::{hitbox::HitBoxCache, rchash::RcHash, vector::Vector, viewbox::ViewBoxCache};

use uuid::Uuid;

use self::implement::position::PositionStatus;

use super::render::Texture;

pub mod implement;

#[derive(Debug)]
pub struct EnityStatus {
  pub name: &'static str,
  pub position: PositionStatus,
  pub uuid: Uuid,
  size: Vector,
  angle: f32,
  display: bool,
  collidable: bool,
  group: HashSet<String>,
  ///
  texture: Texture,
  destroy: bool,

  cache: (ViewBoxCache, HitBoxCache),
}

impl EnityStatus {
  pub fn new(name: &'static str) -> EnityBuilder {
    EnityBuilder {
      name,
      angle: 0.0,
      speed: 50.0,
      display: true,
      collidable: true,
      group: HashSet::new(),
      position: Vector::ORIGIN,
      texture: Texture::default(),
      size: Vector::new(50., 50.),
    }
  }
  pub fn destroy(&mut self) {
    self.destroy = true
  }
  pub fn is_destroy(&self) -> bool {
    self.destroy
  }
  pub fn display(&mut self, value: bool) {
    self.display = value;
  }
  pub fn is_display(&self) -> bool {
    self.display
  }
  pub fn collidable(&mut self, value: bool) {
    self.collidable = value;
  }
  pub fn is_collidable(&self) -> bool {
    self.collidable
  }
  pub fn add_group(&mut self, value: String) {
    self.group.insert(value);
  }
  pub fn remove_group(&mut self, value: String) {
    self.group.remove(&value);
  }
  pub fn has_group(&self, value: String) {
    self.group.contains(&value);
  }
}

pub struct EnityBuilder {
  name: &'static str,
  position: Vector,
  size: Vector,
  angle: f32,
  speed: f32,
  display: bool,
  collidable: bool,
  texture: Texture,
  group: HashSet<String>,
}
impl EnityBuilder {
  pub fn set_position(mut self, position: Vector) -> Self {
    self.position = position;
    self
  }
  pub fn set_size(mut self, size: Vector) -> Self {
    self.size = size;
    self
  }
  pub fn set_angle(mut self, angle: f32) -> Self {
    self.angle = angle;
    self
  }
  pub fn set_speed(mut self, speed: f32) -> Self {
    self.speed = speed;
    self
  }
  pub fn set_display(mut self, display: bool) -> Self {
    self.display = display;
    self
  }
  pub fn set_collidable(mut self, collidable: bool) -> Self {
    self.collidable = collidable;
    self
  }
  pub fn set_texture(mut self, texture: Texture) -> Self {
    self.texture = texture;
    self
  }
  pub fn set_groups(mut self, groups: Vec<&'static str>) -> Self {
    let groups = groups.iter().map(|v| v.to_string());
    self.group = HashSet::from_iter(groups);
    self
  }

  pub fn finished(self) -> EnityStatus {
    EnityStatus {
      name: self.name,
      angle: self.angle,
      size: self.size,
      position: PositionStatus::new(self.position, self.speed),
      collidable: self.collidable,
      texture: self.texture,
      display: self.display,
      group: self.group,
      destroy: false,
      uuid: uuid::Uuid::new_v4(),
      cache: (ViewBoxCache::new(), HitBoxCache::new()),
    }
  }
}

///
///
///
impl Hash for EnityStatus {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.uuid.hash(state)
  }
}

pub trait Enity: Debug {
  fn base(&self) -> RcHash<EnityStatus>;
  fn name(&self) -> &'static str {
    self.base().name
  }
}

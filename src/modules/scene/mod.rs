use std::mem;

use indexmap::{
  map::{Iter, IterMut, Keys, Values, ValuesMut},
  IndexMap,
};
use uuid::Uuid;

use crate::utils::{rect::Rect, vector::Vector};

use self::utils::{grid::Grid, simple_grid::SimpleGrid};

use super::{
  context::render::{Render, RenderFrame, Texture, ViewPort},
  enity::track::EnityTrack,
};

pub mod utils;

pub trait Scene: Render {
  fn render(&self, render: RenderFrame) -> RenderFrame;
}

///=========================================================================================
///
///=========================================================================================
/// # 最基礎的場景系統
/// 負責掛載所有的內容
/// 並提供瀏覽和操作的 API
#[derive(Debug, Clone)]
pub struct NormalScene {
  entities: IndexMap<Uuid, EnityTrack>,
  background: Texture,
  viewport: ViewPort,
  grid: SimpleGrid,
  size: Vector,
  uuid: Uuid,
  ui: UIs,
}

impl NormalScene {
  //   /// 創建一個新的場景
  pub fn new(size: Vector) -> Self {
    NormalScene {
      background: Texture::default(),
      viewport: ViewPort::new(),
      entities: IndexMap::new(),
      uuid: Uuid::new_v4(),
      grid: SimpleGrid::new(),
      ui: UIs::new(),
      size,
    }
  }

  pub fn uuid(&self) -> Uuid {
    self.uuid
  }

  pub fn replace_viewport(&mut self, viewport: ViewPort) -> ViewPort {
    mem::replace(&mut self.viewport, viewport)
  }

  pub fn viewport(&self) -> &ViewPort {
    &self.viewport
  }

  pub fn viewport_mut(&mut self) -> &mut ViewPort {
    &mut self.viewport
  }

  pub fn insert(&mut self, enity: &EnityTrack) {
    self.entities.insert(enity.uuid(), enity.clone());
  }

  pub fn insert_ui(&mut self, enity: &EnityTrack) {
    self.ui.insert(enity);
  }

  pub fn get(&mut self, uuid: &Uuid) -> Option<&EnityTrack> {
    self.entities.get(uuid)
  }

  pub fn get_ui(&mut self, uuid: &Uuid) -> Option<&EnityTrack> {
    self.ui.get(uuid)
  }

  pub fn remove(&mut self, enity: &EnityTrack) {
    self.entities.swap_remove(&enity.uuid());
  }

  pub fn remove_ui(&mut self, enity: &EnityTrack) {
    self.ui.remove(enity);
  }

  pub fn grid(&self) -> &SimpleGrid {
    &self.grid
  }
  pub fn grid_mut(&mut self) -> &mut SimpleGrid {
    &mut self.grid
  }

  pub fn background(&self) -> Texture {
    self.background.clone()
  }

  pub fn set_background(&mut self, texture: Texture) -> Texture {
    mem::replace(&mut self.background, texture)
  }
}

impl NormalScene {
  pub fn update(&mut self, delta: usize) {
    
    let mut tracks = IndexMap::with_capacity(self.entities.len());
    let old_tracks = mem::take(&mut self.entities);
    
    self.grid.clear();
    for (uuid, track) in old_tracks {
      if track.base().is_destroy() {
        continue;
      };
      self.grid.insert(self.uuid, &track);
      tracks.insert(uuid, track.clone());
    }

    for track in tracks.values() {
      calc_collision(self, &track);
    }

    for track in tracks.values() {
        // apply action
        let mut position = track.position(self.uuid);
        position.action(self.uuid, track.base().speed(), delta);
    }
    mem::swap(&mut self.entities, &mut tracks);

    fn calc_collision(scene: &mut NormalScene, track: &EnityTrack) {
      // apply collision
      let collision = track.base().get_collision();
      if collision != 0 {
        let others = scene.grid.collision(scene.uuid, track.clone());

        for other in others {
          let other_collision = other.base().get_collision();
          if collision != other_collision {
            continue;
          };
          
          let mut pos = track.position(scene.uuid);
          let pos_vec = pos.get();
          
          let other_pos_vec = other.position(scene.uuid).get();
          
          let goto = other_pos_vec.to(pos_vec);
          pos.offset(goto / goto.distance() * 2., 0.);
        }
      }
    }
  }

  pub fn collision(&self, enity: &EnityTrack) -> Vec<EnityTrack> {
    self.grid.collision(self.uuid, enity.clone())
  }

  pub fn collision_by_rect(&self, rect: Rect) -> Vec<EnityTrack> {
    self.grid.collision_by_rect(self.uuid, rect)
  }

  pub fn collision_by_point(&self, point: Vector) -> Vec<EnityTrack> {
    self
      .grid
      .collision_by_rect(self.uuid, Rect::new(point, Vector::ORIGIN))
  }
}

impl Render for NormalScene {
  fn render(&self, frame: &mut RenderFrame) {
    let background = (
      Rect::new(Vector::ORIGIN, self.size),
      self.background.clone(),
    );
    frame.push(background);

    for enity in self.entities.values() {
      let enity_position = enity.viewbox_object(self.uuid);
      frame.append(&enity_position);
    }

    for enity in self.ui.values() {
      let enity_position = enity.viewbox_object(self.uuid);
      frame.append(&enity_position);
    }

    frame.set_viewport(self.viewport)
  }
}

#[derive(Debug, Clone)]
pub struct UIs(Uuid, IndexMap<Uuid, EnityTrack>);

impl UIs {
  pub fn new() -> Self {
    Self(Uuid::new_v4(), IndexMap::new())
  }
  pub fn render(&self, frame: &mut RenderFrame) {
    for ui in self.1.values() {
      let enity_position = ui.viewbox_object(self.0);
      frame.append(&enity_position);
    }
  }
  pub fn insert(&mut self, enity: &EnityTrack) {
    self.1.insert(enity.uuid(), enity.clone());
  }
  pub fn remove(&mut self, enity: &EnityTrack) {
    self.1.shift_remove(&enity.uuid());
  }

  pub fn iter(&self) -> Iter<'_, Uuid, EnityTrack> {
    self.1.iter()
  }

  pub fn keys(&self) -> Keys<'_, Uuid, EnityTrack> {
    self.1.keys()
  }

  pub fn values(&self) -> Values<'_, Uuid, EnityTrack> {
    self.1.values()
  }

  pub fn iter_mut(&mut self) -> IterMut<'_, Uuid, EnityTrack> {
    self.1.iter_mut()
  }

  pub fn values_mut(&mut self) -> ValuesMut<'_, Uuid, EnityTrack> {
    self.1.values_mut()
  }

  pub fn get(&self, uuid: &Uuid) -> Option<&EnityTrack> {
    self.1.get(uuid)
  }

  pub fn get_mut(&mut self, uuid: &Uuid) -> Option<&mut EnityTrack> {
    self.1.get_mut(uuid)
  }

  pub fn raw(&self) -> &IndexMap<Uuid, EnityTrack> {
    &self.1
  }
  pub fn raw_mut(&mut self) -> &mut IndexMap<Uuid, EnityTrack> {
    &mut self.1
  }
}

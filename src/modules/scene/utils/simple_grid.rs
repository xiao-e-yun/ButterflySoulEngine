use std::{fmt::Debug, mem};

use crate::{
  modules::{context::control, enity::track::EnityTrack},
  utils::{rect::Rect, vector::Vector, viewbox::ViewBox},
};
use indexmap::{IndexMap, IndexSet};
use uuid::Uuid;

static GRID_SIZE: f32 = 100.;

#[derive(Debug, Clone)]
pub struct SimpleGrid {
  chunks: IndexMap<(isize, isize), Vec<EnityTrack>>,
}

impl SimpleGrid {
  pub fn new() -> Self {
    Self {
      chunks: IndexMap::new(),
    }
  }
  pub fn insert(&mut self, scene_uuid: Uuid, enity: &EnityTrack) {
    let [xs, ys] = Self::detection(enity.hitbox(scene_uuid));

    for x in xs.iter() {
      for y in ys.iter() {
        self
          .chunks
          .entry((*x, *y))
          .or_default()
          .push(enity.clone());
      }
    }
  }
  pub fn clear(&mut self) {
    let mut remove_chunks = vec![];
    for (&idx, chunk) in self.chunks.iter_mut() {
      if chunk.is_empty() {remove_chunks.push(idx)};
      chunk.clear();
    }
    for idx in remove_chunks.iter() {
      self.chunks.swap_remove(idx);
    }
  }

  pub fn collision(&self, scene_uuid: Uuid, enity: EnityTrack) -> Vec<EnityTrack> {
    let [xs, ys] = Self::detection(enity.hitbox(scene_uuid));
    let mut collecter = IndexSet::with_capacity(xs.len() * ys.len() * 5);

    for x in xs.iter() {
      for y in ys.iter() {
        collecter.extend(
          self
            .chunks
            .get(&(*x, *y))
            .unwrap_or(&vec![])
            .iter().filter_map(|other|{
              if &enity == other { return None; };
              if enity.collision(scene_uuid, other) {
                Some(other.clone())
              } else {
                None
              }
            }),
        );
      }
    }

    collecter.into_iter().collect()
  }
  pub fn collision_by_rect(&self, scene_uuid: Uuid, rect: Rect) -> Vec<EnityTrack> {
    let [xs, ys] = Self::detection(rect);
    let mut collecter = IndexSet::with_capacity(xs.len() * ys.len() * 5);

    for x in xs.iter() {
      for y in ys.iter() {
        collecter.extend(
          self
            .chunks
            .get(&(*x, *y))
            .unwrap_or(&vec![])
            .iter().filter_map(|other|{
              if other.collision_node(scene_uuid, rect) {
                Some(other.clone())
              } else {
                None
              }
            }),
        );
      }
    }

    collecter.into_iter().collect()
  }

  fn detection(rect: Rect) -> [Vec<isize>; 2] {
    let (max_pos, min_pos) = rect.maxmin();
    let result = [(max_pos.0, min_pos.0), (max_pos.1, min_pos.1)].map(|(max, min)| {
      let max = (max / GRID_SIZE).floor() as isize;
      let min = (min / GRID_SIZE).floor() as isize;
      (min..=max).collect::<Vec<isize>>()
    });
    result
  }
}
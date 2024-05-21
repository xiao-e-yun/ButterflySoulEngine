use std::{fmt::Debug, mem};

use crate::{
  modules::enity::track::EnityTrack,
  utils::{rect::Rect, vector::Vector, viewbox::ViewBox},
};
use indexmap::{IndexMap, IndexSet};
use uuid::Uuid;

static GRID_SIZE: f32 = 1200.;
static GRID_SUB_SIZE: f32 = GRID_SIZE / 3.;
static GRID_BLOCK_SIZE: f32 = GRID_SUB_SIZE / 2.;
static GRID_CAPACITY: usize = 4;

#[derive(Debug, Clone)]
pub struct Grid {
  chunks: IndexMap<(isize, isize), GridChunk>,
}

impl Grid {
  pub fn new() -> Self {
    Self {
      chunks: IndexMap::new(),
    }
  }
  pub fn insert(&mut self, scene_uuid: Uuid, enity: &EnityTrack) {
    let [xs, ys] = Self::detection(enity.viewbox(scene_uuid));

    for x in xs.iter() {
      for y in ys.iter() {
        self
          .chunks
          .entry((*x, *y))
          .or_default()
          .insert(scene_uuid, enity.clone());
      }
    }
  }
  pub fn clear(&mut self) {
    let mut remove_chunks = vec![];
    for (&idx, chunk) in self.chunks.iter_mut() {
      let is_empty = chunk.clear();
      if is_empty {
        remove_chunks.push(idx)
      }
    }
    for idx in remove_chunks.iter() {
      self.chunks.swap_remove(idx);
    }
  }
  pub fn collision(&self, scene_uuid: Uuid, enity: EnityTrack) -> Vec<EnityTrack> {
    let [xs, ys] = Self::detection(enity.viewbox(scene_uuid));
    let chunk = Rect::new(Vector::new(GRID_SIZE, GRID_SIZE) / 2., Vector::new(GRID_SIZE, GRID_SIZE));
    let mut collecter = IndexSet::with_capacity(xs.len() * ys.len() * 5);

    for x in xs.iter() {
      for y in ys.iter() {
        let mut chunk = chunk.clone();
        chunk.position *= (Vector::new(*x as f32, *y as f32) + 0.5) * GRID_SIZE;
        collecter.extend(
          self
            .chunks
            .get(&(*x, *y))
            .unwrap_or(&GridChunk::default())
            .collect(scene_uuid, enity.clone(), chunk),
        );
      }
    }

    collecter
      .iter()
      .filter_map(|other| {
        if &enity == other {
          return None;
        }

        if enity.collision(scene_uuid, other) {
          Some(other.clone())
        } else {
          None
        }
      })
      .collect()
  }
  pub fn collision_by_rect(&self, scene_uuid: Uuid, rect: Rect) -> Vec<EnityTrack> {
    let [xs, ys] = Self::detection(rect);
    let chunk = Rect::new(Vector::ORIGIN, Vector::new(GRID_SIZE, GRID_SIZE));
    let mut collecter = IndexSet::with_capacity(xs.len() * ys.len() * 5);

    for x in xs.iter() {
      for y in ys.iter() {
        let mut chunk = chunk.clone();
        chunk.position *= (Vector::new(*x as f32, *y as f32) + 0.5) * GRID_SIZE;
        collecter.extend(
          self
            .chunks
            .get(&(*x, *y))
            .unwrap_or(&GridChunk::default())
            .collect_node(rect, chunk),
        );
      }
    }

    let mut result = Vec::with_capacity(collecter.len());
    for node in collecter {
      if node.collision_node(scene_uuid, rect) {
        result.push(node)
      }
    }
    result
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

#[derive(Debug, Clone)]
pub enum GridChunk {
  /// 1200
  Grid(Box<[GridChunk; 9]>, Vec<EnityTrack>),
  /// 400
  SubGrid(Box<[GridChunk; 4]>, Vec<EnityTrack>),
  /// 200 or 400 or 1200
  Block(Vec<EnityTrack>),
}

impl GridChunk {
  pub fn insert(&mut self, scene_uuid: Uuid, enity: EnityTrack) {
    self.insert_node(scene_uuid, enity, 0)
  }

  pub fn clear(&mut self) -> bool {
    let mut is_empty = true;
    match self {
      GridChunk::Grid(chunks, list) => {
        for chunk in chunks.iter_mut() {
          is_empty = chunk.clear() && list.is_empty() && is_empty
        }
        list.clear()
      }
      GridChunk::SubGrid(chunks, list) => {
        for chunk in chunks.iter_mut() {
          is_empty = chunk.clear() && list.is_empty() && is_empty
        }
        list.clear()
      }
      GridChunk::Block(list) => {
        is_empty = list.is_empty();
        list.clear()
      }
    }
    is_empty
  }

  pub fn collect(&self, scene_uuid: Uuid, enity: EnityTrack, chunk: Rect) -> Vec<EnityTrack> {
    let viewbox = enity.viewbox(scene_uuid);
    self.collect_node(viewbox, chunk)
  }

  fn collect_node(&self, viewbox: Rect, chunk: Rect) -> Vec<EnityTrack> {
    let mut collecter: Vec<EnityTrack> = vec![];

    match self {
      GridChunk::Grid(chunks, list) => {
        //collect current node
        collecter.append(&mut list.clone());
        //collect children node
        collecter.append(&mut collect_chunks(
          chunks.as_slice(),
          chunk,
          viewbox,
          3,
          GRID_SUB_SIZE,
        ));
      }
      GridChunk::SubGrid(chunks, list) => {
        //collect current node
        collecter.append(&mut list.clone());
        //collect children node
        collecter.append(&mut collect_chunks(
          chunks.as_slice(),
          chunk,
          viewbox,
          2,
          GRID_BLOCK_SIZE,
        ));
      }
      GridChunk::Block(list) => collecter.append(&mut list.clone()),
    }

    fn collect_chunks(
      chunks: &[GridChunk],
      parent_chunk: Rect,
      viewbox: Rect,
      width: usize,
      size: f32,
    ) -> Vec<EnityTrack> {
      let mut collecter = vec![];
      let mut parent_chunk = parent_chunk.clone();
      parent_chunk.position -= parent_chunk.size / 2.;
      parent_chunk.size = Vector::new(size, size);
      for (i, grid_chunk) in chunks.iter().enumerate() {
        let mut chunk_box = parent_chunk.clone();
        chunk_box.position +=
          Vector::new((i % width) as f32 + 0.5, (i / width) as f32 + 0.5) * size;
        let collision = chunk_box.collision(&viewbox);

        if collision {
          collecter.append(&mut grid_chunk.collect_node(viewbox, chunk_box))
        }
      }
      collecter
    }

    collecter
  }

  fn insert_node(&mut self, scene_uuid: Uuid, enity: EnityTrack, depth: usize) {
    match self {
      GridChunk::Grid(chunk, list) => {
        let chunks_width = 3_usize;
        let points =
          get_points(scene_uuid, &enity).map(|p| get_chunck_place(p, chunks_width, GRID_SUB_SIZE));
        let idx = check_in_single_chunck(&points);
        match idx {
          Some((x, y)) => {
            chunk
              .get_mut(x + y * chunks_width)
              .unwrap()
              .insert_node(scene_uuid, enity, depth + 1)
          }
          None => list.push(enity),
        }
      }
      GridChunk::SubGrid(chunk, list) => {
        let chunks_width = 2_usize;
        let points = get_points(scene_uuid, &enity)
          .map(|p| get_chunck_place(p, chunks_width, GRID_BLOCK_SIZE));
        let idx = check_in_single_chunck(&points);
        match idx {
          Some((x, y)) => {
            chunk
              .get_mut(x + y * chunks_width)
              .unwrap()
              .insert_node(scene_uuid, enity, depth + 1)
          }
          None => list.push(enity),
        }
      }
      GridChunk::Block(list) => {
        if depth < 2 && (2 - depth) * GRID_CAPACITY == list.len() {
          let mut target = if depth == 0 {
            GridChunk::Grid(Box::default(), vec![])
          } else {
            GridChunk::SubGrid(Box::default(), vec![])
          };
          for track in list.clone() {
            target.insert_node(scene_uuid, track, depth)
          }
          target.insert_node(scene_uuid, enity, depth);
          mem::swap(self, &mut target);
        } else {
          list.push(enity)
        }
      }
    }

    fn check_in_single_chunck(points: &[(usize, usize)]) -> Option<(usize, usize)> {
      let first = points.first().unwrap();
      for point in points {
        if first.0 != point.0 || first.1 != point.1 {
          return None;
        }
      }
      Some(first.clone())
    }

    // 無法處理跨區塊
    fn get_chunck_place(point: Vector, width: usize, size: f32) -> (usize, usize) {
      let place = (point / size).floor();
      let signs = place.signum().min(Vector::ORIGIN);
      let (x, y) = (place.abs() + signs).unpack();
      (x as usize % width, y as usize % width)
    }

    fn get_points(scene_uuid: Uuid, enity: &EnityTrack) -> [Vector; 4] {
      enity.viewbox(scene_uuid).points()
    }
  }
}

impl Default for GridChunk {
  fn default() -> Self {
    GridChunk::Block(vec![])
  }
}

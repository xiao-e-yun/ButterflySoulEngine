use std::mem;

use uuid::Uuid;

use crate::utils::vector::Vector;

use super::track::EnityTrack;

#[derive(Debug, Clone, Default)]
pub struct EnityPosition {
  main: MoveEvent,
  drifts: Vec<(Vector, f32)>,
  position: Vector,
  angle: f32,
}

impl EnityPosition {
  pub fn new(position: Vector) -> Self {
    EnityPosition {
      position,
      angle: 0.0,
      main: MoveEvent::Stop,
      drifts: vec![],
    }
  }

  pub fn set_angle(&mut self, angle: f32) {
    self.angle = angle;
  }

  pub fn get_angle(&self) -> f32 {
    self.angle
  }

  pub fn set_action(&mut self, event: MoveEvent) {
    self.main = event;
  }

  pub fn get_action(&self) -> &MoveEvent {
    &self.main
  }

  pub fn offset(&mut self, offset: Vector, spend: f32) {
    self.drifts.push((offset, spend));
  }

  pub fn get(&self) -> Vector {
    self.position
  }

  pub fn set(&mut self, position: Vector) -> Vector {
    mem::replace(&mut self.position, position)
  }

  pub fn action(&mut self, scene_uuid: Uuid, speed: f32, delta: usize) {
    let mut position = self.position;
    let mut force_drift = false;
    let delta = delta as f32 / 1000.;
    match &mut self.main {
      MoveEvent::Stop => {
        //notthing to do
      }
      MoveEvent::Moveto(target) => {
        let moving = position.to(target.clone());
        position += if moving.distance() > speed * delta {
          moving.by_length(speed) * delta
        } else {
          moving
        }
      }
      MoveEvent::Moving(target) => {
        position += target.by_length(speed) * delta;
      }
      MoveEvent::Drift(vector, spend) => {
        force_drift = true;
        if *spend > f32::EPSILON {
          let used = delta.min(*spend).max(0.);
          let left = *spend - used;
          let scale = *vector * (used / *spend);
          position += scale;
          *vector -= scale;
          *spend = left;
        }
      }
      MoveEvent::Following(enity) => {
        position += if !enity.base().is_destroy() {
          let vector = enity.position(scene_uuid).get();
          let goto = position.to(vector);
          goto.by_length(speed) * delta
        } else {
          Vector::ORIGIN
        }
      }
    }

    //drift
    let mut offset = Vector::ORIGIN;
    if self.drifts.len() != 0 {
      self.drifts.retain_mut(|(vector, total)| {
        if *total < f32::EPSILON {
          offset += *vector;
          return false;
        }

        let used = delta.min(*total).max(0.);
        let left = *total - used;
        let scale = *vector * used / *total;

        offset += scale;
        *vector -= scale;
        *total = left;

        left > f32::EPSILON //unfinished if true
      });
    }
    if !force_drift {
      position += offset;
    }

    self.position = position
  }
}

#[derive(Debug, Clone)]
pub enum MoveEvent {
  Stop,
  ///by absolute position
  Moveto(Vector),
  ///by relative position
  Moving(Vector),
  Drift(Vector, f32),
  Following(EnityTrack),
}

impl Default for MoveEvent {
  fn default() -> Self {
    MoveEvent::Stop
  }
}

use std::{
  cell::Cell,
  rc::{Rc, Weak},
};

use crate::{modules::enity::EnityStatus, utils::vector::Vector};

#[derive(Debug, Clone)]
pub struct PositionStatus {
  main: MoveEvent,
  drifts: Vec<(Vector, f32, f32, bool)>,
  position: Rc<Cell<Vector>>,
  pub base: f32,
}

impl PositionStatus {
  pub fn new(position: Vector, base: f32) -> Self {
    PositionStatus {
      position: Rc::new(Cell::new(position)),
      drifts: vec![],
      main: MoveEvent::Stop,
      base,
    }
  }

  pub fn set_action(&mut self, event: MoveEvent) {
    self.main = event;
  }

  pub fn drift(&mut self, offset: Vector, spend: f32) {
    self.drifts.push((offset, spend, spend, false));
  }

  pub fn drift_with_break(&mut self, offset: Vector, spend: f32) {
    self.drifts.push((offset, spend, spend, true));
  }
  
  pub fn breaking(&mut self, spend: f32) {
    self.drifts.push((Vector::ORIGIN, 0., spend, true));
  }

  pub fn get(&self) -> Vector {
    self.position.get()
  }

  pub fn set(&self, value: Vector) {
    self.position.set(value);
  }

  pub fn track(&self) -> Weak<Cell<Vector>> {
    Rc::downgrade(&self.position.clone())
  }

  pub fn action(&mut self, time: f32) {

    //drift
    let mut break_action = false;
    self.drifts.retain_mut(|(vector, total, left, breaking)| {
      if *breaking { break_action = true }

      *left -= time;
      let finished = *left < f32::EPSILON;
      let time = if finished { time + *left } else { time };
      if *total > 0.0 {
        self
          .position
          .set(self.position.get() + *vector / *total * time);
      }
      !finished
    });


    //action
    if !break_action {
      let self_position = self.position.get();
      self.position.set(
        self_position
          + match &self.main {
            MoveEvent::Moveto(position) => {
              let moving = self_position.to(position.clone());
              if moving.distance() > self.base * time {
                moving.by_length(self.base) * time
              } else {
                moving
              }
            }
            MoveEvent::Moving(position) => position.by_length(self.base) * time,
            MoveEvent::Following(enity) => {
              let vector = enity.upgrade();
              if let Some(vector) = vector {
                let goto = self_position.to(vector.get());
                goto.by_length(self.base) * time
              } else {
                Vector::ORIGIN
              }
            }
            MoveEvent::Stop => Vector::ORIGIN,
          },
      );
    }
  }
}

#[derive(Debug, Clone)]
pub enum MoveEvent {
  Stop,
  ///by absolute position
  Moveto(Vector),
  ///by relative position
  Moving(Vector),
  Following(Weak<Cell<Vector>>),
}

impl EnityStatus {
  pub fn moveto(&mut self, target: Vector) {
    let event = MoveEvent::Moveto(target);
    self.position.set_action(event);
  }
  pub fn following(&mut self, target: Weak<Cell<Vector>>) {
    let event = MoveEvent::Following(target);
    self.position.set_action(event);
  }
  pub fn moving(&mut self, vector: Vector) {
    let event = MoveEvent::Moving(vector);
    self.position.set_action(event)
  }
  pub fn stop(&mut self) {
    self.position.set_action(MoveEvent::Stop)
  }
  pub fn get_position(&self) -> Vector {
    self.position.get()
  }
  pub fn set_position(&mut self, position: Vector) {
    self.position.set(position)
  }
}

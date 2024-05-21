use std::{cell::{Ref, RefMut}, fmt::Debug};

use uuid::Uuid;

use self::{base::EnityBase, position::EnityPosition, track::EnityTrack, view::EnityView};

pub mod base;
pub mod position;
pub mod track;
pub mod view;

pub trait Enity: Debug {
  fn track(&self) -> &EnityTrack;

  fn base(&self) -> Ref<EnityBase> {
    self.track().base()
  }

  fn view(&self) -> Ref<EnityView> {
    self.track().view()
  }

  fn position(&self,uuid: Uuid) -> RefMut<EnityPosition> {
    self.track().position(uuid)
  }

  fn base_mut(&mut self) -> RefMut<EnityBase> {
    self.track().base_mut()
  }

  fn view_mut(&mut self) -> RefMut<EnityView> {
    self.track().view_mut()
  }

  fn uuid(&self) -> Uuid {
    self.track().uuid()
  }
}

impl PartialEq for dyn Enity {
  fn eq(&self, other: &Self) -> bool {
    self.uuid() == other.uuid()
  }
}

// impl Enity {
//   pub fn new() -> EnityBuilder {
//     EnityBuilder::new()
//   }

//   // pub fn collision(&self, other: &RcHash<Enity>) -> bool {
//   //   let borrowed = other.borrow();
//   //   for (hitbox, _) in self.hitboxes.iter() {
//   //     for (other_hitbox, _) in borrowed.hitboxes.iter() {
//   //       if hitbox.collision(other_hitbox) {
//   //         return true;
//   //       }
//   //     }
//   //   }
//   //   return false;
//   // }

//   //
//   // uuid
//   //

//   pub fn uuid(&self) -> Uuid {
//     self.base.uuid
//   }

//   pub fn check_uuid(&self, uuid: Uuid) -> bool {
//     self.base.uuid == uuid
//   }
// }

// impl Render for Enity {
//   fn render(&self, frame: &mut RenderFrame) {
//     frame.append(&self.textures);
//     frame.append(&self.hitboxes);
//   }
// }

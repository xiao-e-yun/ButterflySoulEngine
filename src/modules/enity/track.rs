use std::{cell::RefMut, fmt::Debug, hash::Hash};

use indexmap::IndexMap;
use std::cell::Ref;
use uuid::Uuid;

use crate::{modules::context::render::Texture, utils::{hitbox::HitBox, rchash::RcHash, rect::Rect, vector::Vector, viewbox::ViewBox}};

use super::{base::EnityBase, position::EnityPosition, view::EnityView};

// #[derive(Debug, Clone)]
#[derive(Clone)]
pub struct EnityTrack {
  uuid: Uuid,
  position: RcHash<IndexMap<Uuid, EnityPosition>>,
  base: RcHash<EnityBase>,
  view: RcHash<EnityView>,
}

impl Debug for EnityTrack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnityTrack").finish()
    }
}

impl PartialEq for EnityTrack {
  fn eq(&self, other: &Self) -> bool {
    self.uuid == other.uuid
  }
}

impl Hash for EnityTrack {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.uuid.hash(state);
  }
}

impl Eq for EnityTrack {}

impl EnityTrack {
  pub fn new(base: EnityBase, view: EnityView) -> EnityTrack {
    EnityTrack {
      uuid: Uuid::new_v4(),
      base: RcHash::new(base),
      view: RcHash::new(view),
      position: RcHash::default(),
    }
  }

  pub fn uuid(&self) -> Uuid {
    self.uuid
  }

  pub fn base(&self) -> Ref<EnityBase> {
    self.base.borrow()
  }
  pub fn base_mut(&self) -> RefMut<EnityBase> {
    self.base.borrow_mut()
  }

  pub fn view(&self) -> Ref<EnityView> {
    self.view.borrow()
  }
  pub fn view_mut(&self) -> RefMut<EnityView> {
    self.view.borrow_mut()
  }

  pub fn position(&self, uuid: Uuid) -> RefMut<EnityPosition> {
    RefMut::map(self.position.borrow_mut(), |s| s.entry(uuid).or_default())
  }
}

impl EnityTrack {
  pub fn viewbox(&self,scene_uuid: Uuid) -> Rect {
    let viewboxes = self.viewbox_object(scene_uuid);

    if viewboxes.len() == 0 {
      return Rect::new(Vector::ORIGIN, Vector::ORIGIN)
    }

    let mut max = Vector::new(f32::MIN, f32::MIN);
    let mut min = Vector::new(f32::MAX, f32::MAX);

    let maxmin = viewboxes.iter().map(|(rect,_)|rect.maxmin());
    for (max_point, min_point) in maxmin {
      min = min.min(min_point);
      max = max.max(max_point);
    }

    let size = max - min;
    let center = size / 2. + min;

    Rect::new(center, size)
  }
 
  pub fn hitbox(&self,scene_uuid: Uuid) -> Rect {
    let hitbox_object = self.hitbox_object(scene_uuid);

    if hitbox_object.len() == 0 {
      return Rect::new(Vector::ORIGIN, Vector::ORIGIN)
    }

    let mut max = Vector::new(f32::MIN, f32::MIN);
    let mut min = Vector::new(f32::MAX, f32::MAX);

    let maxmin = hitbox_object.iter().map(|rect|rect.maxmin());
    for (max_point, min_point) in maxmin {
      min = min.min(min_point);
      max = max.max(max_point);
    }

    let size = max - min;
    let center = size / 2. + min;

    Rect::new(center, size)
  }

  pub fn viewbox_object(&self, scene_uuid: Uuid) -> Vec<(Rect,Texture)> {
    let position = self.position(scene_uuid);
    let view = self.view();

    let angle = position.get_angle();
    let offset = position.get();
    let mut viewboxes = view.viewboxes();
    for (rect, _) in viewboxes.iter_mut() {
      rect.angle = angle;
      rect.position = rect.position.rotate(angle) + offset;
    }
    viewboxes
  }
}

impl EnityTrack {
  pub fn collision(&self, scene_uuid: Uuid, other: &Self) -> bool {
    let self_obj = self.hitbox_object(scene_uuid);
    let other_obj = other.hitbox_object(scene_uuid);

    for srect in self_obj.iter() {
      for orect in other_obj.iter() {
        if HitBox::collision(srect, orect) {
          return true;
        };
      }
    }

    false
  }

  pub fn collision_node(&self, scene_uuid: Uuid, other: Rect) -> bool {
    let self_obj = self.hitbox_object(scene_uuid);

    for srect in self_obj.iter() {
      if HitBox::collision(srect, &other) {
        return true;
      };
    }

    false
  }

  pub fn hitbox_object(&self, scene_uuid: Uuid) -> Vec<Rect> {
    let position = self.position(scene_uuid);
    let view = self.view();
    let angle = position.get_angle();
    let offset = position.get();
    let mut hitboxes = view.hitboxes();
    for rect in hitboxes.iter_mut() {
      rect.angle = angle;
      rect.position = rect.position.rotate(angle) + offset;
    }
    hitboxes
  }
}

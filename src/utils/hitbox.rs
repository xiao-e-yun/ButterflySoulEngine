use crate::utils::vector::Vector;

use super::viewbox::ViewBox;

pub trait HitBox: ViewBox {
  fn angle(&self) -> f32;
  fn size(&self) -> Vector;
  fn position(&self) -> Vector;

  fn nomral_vector(&self) -> [Vector; 2] {
    let [lt, rt, rb, _] = self.points();
    let nomrals = [lt.to(rt).normal(), rt.to(rb).normal()];

    nomrals
  }

  fn collision(&self, other: &impl HitBox) -> bool {
    if HitBox::angle(self).abs() < f32::EPSILON && HitBox::angle(other).abs() < f32::EPSILON {
      return ViewBox::collision(self, other);
    }

    let mut a = vec![];
    for nomral in self.nomral_vector() {
      let mut max = f32::MIN;
      let mut min = f32::MAX;
      for point in self.points() {
        let orthoprojection = nomral.orthoprojection_length(point);
        if orthoprojection > max {
          max = orthoprojection
        }
        if orthoprojection < min {
          min = orthoprojection
        }
      }
      let mut other_max = f32::MIN;
      let mut other_min = f32::MAX;
      for point in other.points() {
        let orthoprojection = nomral.orthoprojection_length(point);
        if orthoprojection > other_max {
          other_max = orthoprojection
        }
        if orthoprojection < other_min {
          other_min = orthoprojection
        }
      }

      a.push(nomral);

      if max < other_min || min > other_max {
        return false;
      }
    }
    true
  }
}
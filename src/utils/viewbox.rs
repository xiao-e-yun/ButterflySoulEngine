use crate::utils::vector::Vector;

pub trait ViewBox {
  fn angle(&self) -> f32;
  fn size(&self) -> Vector;
  fn position(&self) -> Vector;

  fn points(&self) -> [Vector; 4] {
    let position = self.position();
    let size = self.size();
    let angle = self.angle();

    //四個點
    let points = [
      Vector::new(-1., 1.),
      Vector::new(1., 1.),
      Vector::new(1., -1.),
      Vector::new(-1., -1.),
    ]
    .map(|path| position + (size * path).rotate(angle));

    points
  }

  fn maxmin(&self) -> (Vector, Vector) {
    //讀區物件向量
    let points = self.points();
    let mut points = points.iter();
    let mut max = *points.next().unwrap();
    let mut min = max.clone();

    for point in points {
      if point.0 > max.0 {
        max.0 = point.0
      };
      if point.1 > max.1 {
        max.1 = point.1
      };
      if point.0 < min.0 {
        min.0 = point.0
      };
      if point.1 < min.1 {
        min.1 = point.1
      };
    }

    (max, min)
  }
  
  fn collision<T: ViewBox + ?Sized>(&self, other: &T) -> bool {
    //檢測是否碰撞

    let (this_max, this_min) = self.maxmin(); //設置正物件x,y值資訊
    let (other_max, other_min) = other.maxmin(); //設置副物件x,y值資訊

    this_max.0 > other_min.0 &&   //(正物件最高x軸) > (正物件最小x軸) &&
    other_max.0 > this_min.0 &&   //(副物件最高x軸) > (副物件最小x軸) &&
    this_max.1 > other_min.1 &&   //(正物件最高y軸) > (副物件最小y軸) &&
    other_max.1 > this_min.1 //(副物件最高y軸) > (副物件最小y軸)
  }
}

use std::{cell::Cell, cmp::Ordering, mem, slice::Iter, vec};

use cached::proc_macro::cached;
use serde::{Deserialize, Serialize};

use super::vector::Vector;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Polygon {
  nodes: Vec<Vector>,
  offset: Vector,
  sorted: bool,
}

impl Polygon {
  pub fn new() -> Self {
    Self {
      nodes: vec![],
      sorted: false,
      offset: Vector::ORIGIN,
    }
  }

  pub fn get_node(&self, index: usize) -> Option<(Vector, Vector)> {
    self.nodes.get(index)
  }

  pub fn insert_node(&mut self, position: Vector) {
    self.nodes.push(position);
    self.sorted = false;
  }

  pub fn len(&self) -> usize {
    self.nodes.len()
  }

  pub fn to_chain(&self) -> (Vector, Vec<Vector>) {
    let (mut p, v) = to_chain(self.nodes);
    p += self.offset;
    (p, v)
  }

  pub fn sort(&mut self) {
    if self.sorted {
      return;
    };
    self.nodes.sort_by(|a, b| {
      let x = a.0.partial_cmp(&b.0).unwrap();
      let Ordering::Equal = x else { return x };
      a.1.partial_cmp(&b.1).unwrap()
    });
    self.sorted = true;
  }

  pub fn contain(&mut self, vector: &[Vector]) {}

  /// Andrew's Monotone Chain
  pub fn convex_hull(&mut self) -> Vec<Vector> {
    self.sort();

    let mut result = vec![];

    result.extend(get_hull(self.nodes.iter()));
    result.extend(get_hull(self.nodes.iter().rev()));

    fn get_hull(nodes: Iter<Vec<Vector>>) -> Vec<Vector> {
      let mut points = vec![];

      'node: for node in nodes.iter() {
        loop {
          let Some([p1, p2]) = points.last_chunk() else {
            points.push(node);
            continue 'node;
          };

          let last = p1.to(p2);
          let vector = p1.to(node);
          if last.cross(vector) <= 0.0 {
            points.pop();
            continue;
          }

          points.push(node);
          break;
        }
      }

      points
    }

    result
  }
}

#[cached]
pub fn to_chain(mut input: Vec<Vector>) -> (Vector, Vec<Vector>) {
  let mut last = input.pop().unwrap();
  let position = last;

  for node in input.iter_mut().rev() {
    let temp = *node;
    *node = node.to(last);
    last = temp;
  }

  (position, input)
}

#[cached]
pub fn gjk(a: &Polygon, b: &Polygon) -> Polygon {
  let direction = a.offset.to(b.offset);
  let simplex = Polygon::new();
  simplex.insert_node(support(&direction, a, b));
  direction = -direction;

  loop {
    let point = support(&direction, a, b);

    if point.dot(direction) < 0 {
      return false;
    };
    simplex.insert_node(point);

    if simplex.contain(Vector::ORIGIN) {
      return true;
    };

    let [p1, p2] = simplex.nodes.last_chunk().unwrap();
    direction = p1.to(p2).normal();
  }

  #[cached]
  pub fn support(direction: &Vector, a: &Polygon, b: &Polygon) -> Vector {
    let a_max = get_max_by_dot(a, direction);
    let b_max = get_max_by_dot(b, direction);
    a_max - b_max
  }

  #[cached]
  fn get_max_by_dot(polygon: &Polygon, direction: &Vector) -> Vector {
    let mut max = 0.0;
    let mut vector = Vector::ORIGIN;
    for node in polygon.nodes {
      let dis = node.dot(direction);
      if dis > max {
        max = dis;
        vector = node;
      }
    }
    vector
  }

  #[cached]
  fn find_next_direction(simplex: Polygon) -> Vector {
    fn get_perpendicular(a: Vector, b: Vector) -> Vector {
      let origin = -a;
      let vector = a.to(b);
      if vector.cross(origin) >= 0.0 {
        vector.normal()
      } else {
        vector.right_normal()
      }
    }

    // Neg
    -match simplex.len() {
      2 => {
        let [a, b] = simplex.nodes.as_slice();
        get_perpendicular(a, b)
      }
      3 => {
        let [a, b, c] = simplex.nodes.as_slice();

        let corss_ca = get_perpendicular(c,a);
        let corss_cb = get_perpendicular(c,b);

        if corss_ca.distance_magnitude() < corss_cb.distance_magnitude() {
          simplex.remove(1);
          corss_ca
        } else {
          simplex.remove(0);
          corss_cb
        }
      }
      _ => unreachable!(),
    }
  }
}

#[cached]
pub fn epa(input: &Polygon) -> Vector {
  todo!()
}

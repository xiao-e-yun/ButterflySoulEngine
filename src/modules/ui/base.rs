use serde::{Deserialize, Serialize};

use crate::utils::vector::Vector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIBase {
  name: String,
  position: Vector,
  viewbox: Vector, //for cache

  //events
  clickable: bool,
  hoverable: bool,
}

impl UIBase {
  pub fn new(name: String, position: Vector) -> Self {
    Self {
      name,
      position,
      viewbox: Vector::ORIGIN,

      // events
      clickable: false,
      hoverable: false,
    }
  }
}

impl Default for UIBase {
  fn default() -> Self {
    Self::new("Unknow".to_string(), Vector::ORIGIN)
  }
}

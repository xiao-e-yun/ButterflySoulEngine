use crate::utils::vector::Vector;

#[derive(Debug, Clone)]
pub struct Control {
  pub keys: Vec<KeyEvent>,
  pub click: [Option<Vector>; 2],
  pub mouse: Vector,
}

#[derive(Debug, Clone, Hash)]
pub struct KeyEvent {
  pub code: String,
  pub alt: bool,
  pub ctrl: bool,
  pub meta: bool,
  pub shift: bool,
  pub repeat: bool,
}

impl Control {
  pub fn new() -> Control {
    Control {
      keys: vec![],
      click: [None; 2],
      mouse: Vector::ORIGIN,
    }
  }
}

pub trait GetMoveVector {
  fn move_top(&self) -> Vec<String>;
  fn move_bottom(&self) -> Vec<String>;
  fn move_left(&self) -> Vec<String>;
  fn move_right(&self) -> Vec<String>;
  fn keys(&self) -> &Vec<KeyEvent>;

  fn move_vector(&self) -> Vector {
    let keys = self.keys();
    let mut pressed = [false;4];
    let map = [self.move_top(),self.move_bottom(),self.move_left(),self.move_right()];
    for event in keys.iter() {
      for (key_list,is_pressed) in map.iter().zip(pressed.iter_mut()) {
        if !*is_pressed { *is_pressed = key_list.contains(&event.code) }
      }
    }

    fn sign(v: bool) -> f32 { if v { 1. } else { 0. }}

    Vector::new(
      sign(pressed[3]) - sign(pressed[2]),
      sign(pressed[0]) - sign(pressed[1]),
    )
  }
}

impl GetMoveVector for Control {
  fn move_top(&self) -> Vec<String> {
    vec!["KeyW".to_string()]
  }
  
  fn move_bottom(&self) -> Vec<String> {
    vec!["KeyS".to_string()]
  }
  
    fn move_left(&self) -> Vec<String> {
      vec!["KeyA".to_string()]
    }

  fn move_right(&self) -> Vec<String> {
    vec!["KeyD".to_string()]
  }
  
  fn keys(&self) -> &Vec<KeyEvent> {
    &self.keys
  }
}

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{modules::{context::render::Render, ui::base::UIBase}, object_method};

use super::{UIObject, UIObjectTrait};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIContainer {
  base: UIBase,
  items: IndexMap<String, Box<UIObject>>,
}

impl UIContainer {
  pub fn new() -> Self {
    UIContainer {
      base: UIBase::new(name, position),
      items: ()
    }
  }
}


impl Render for UIContainer {
  fn render(&self, frame: &mut crate::modules::context::render::RenderFrame) {
      todo!()
  }
}

impl UIObjectTrait for UIContainer {

}

object_method!(Container:UIContainer);
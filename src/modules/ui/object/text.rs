use serde::{Deserialize, Serialize};

use crate::{modules::{context::render::Render, ui::base::UIBase}, object_method};

use super::{UIObject, UIObjectTrait};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIText {
  base: UIBase,
  text: String,
}

impl Render for UIText {
  fn render(&self, frame: &mut crate::modules::context::render::RenderFrame) {
      todo!()
  }
}

impl UIObjectTrait for UIText {

}

object_method!(Text: UIText);
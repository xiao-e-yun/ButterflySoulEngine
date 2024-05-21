use serde::{Deserialize, Serialize};

use crate::{modules::{context::render::{Render, Texture}, ui::base::UIBase}, object_method};

use super::{UIObject, UIObjectTrait};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIImage {
  base: UIBase,
  image: Texture,
}

impl Render for UIImage {
    fn render(&self, frame: &mut crate::modules::context::render::RenderFrame) {
        todo!()
    }
}

impl UIObjectTrait for UIImage {

}

object_method!(Image:UIImage);
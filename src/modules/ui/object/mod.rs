pub mod container;
pub mod image;
pub mod text;

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::modules::context::render::{Render, RenderFrame};

use self::{container::UIContainer, image::UIImage, text::UIText};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIObject {
  children: 
}
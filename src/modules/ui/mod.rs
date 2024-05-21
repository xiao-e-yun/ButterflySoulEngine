pub mod base;
pub mod event;
pub mod object;
mod macros;

use std::collections::BTreeMap;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::utils::{rect::Rect, vector::Vector};

use self::{base::UIBase, object::{container::UIContainer, UIObject}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UI {
  name: String,
  tree: UIObject,
}
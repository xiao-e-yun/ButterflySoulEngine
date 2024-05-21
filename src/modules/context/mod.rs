use self::{control::Control, render::RenderFrame};

pub mod render;
pub mod control;

pub trait Context {
  fn control(&self) -> Option<Control>;
  fn render(&self, frame: RenderFrame) -> Option<()>;
  // fn save(&self, data: Vec<u8>);
}
use std::{
  cell::{Ref, RefMut},
  fmt::Debug,
  hash::Hash,
};

use indexmap::{
  map::{IntoIter, IntoKeys, IntoValues},
  IndexMap,
};

use super::rchash::RcHash;

#[derive(Debug, Hash)]
pub struct BseMap<T: BseMapNode>(RcHash<IndexMap<T::Key, RcHash<T>>>);

pub trait BseMapNode: Debug {
  type Key: Hash + Clone + Eq;
  fn get_key(&self) -> Self::Key;
}

impl<T: BseMapNode> BseMap<T> {
  fn inner(&self) -> Ref<IndexMap<T::Key, RcHash<T>>> {
    self.0.borrow()
  }

  fn inner_mut(&self) -> RefMut<IndexMap<<T>::Key, RcHash<T>>> {
    self.0.borrow_mut()
  }

  pub fn new() -> Self {
    Self::default()
  }

  pub fn shift_insert(&self, index: usize, node: RcHash<T>) -> RcHash<T> {
    self
      .inner_mut()
      .shift_insert(index, node.borrow().get_key(), node.clone());
    node
  }
  pub fn insert(&self, node: RcHash<T>) -> RcHash<T> {
    let key = node.borrow().get_key();
    self.inner_mut().insert(key, node.clone());
    node
  }

  pub fn get_by_node(&self, node: &RcHash<T>) -> Option<RcHash<T>> {
    self.get(&node.borrow().get_key())
  }
  pub fn get(&self, key: &T::Key) -> Option<RcHash<T>> {
    self.inner().get(key).cloned()
  }

  pub fn swap_remove_by_node(&self, node: &RcHash<T>) -> Option<RcHash<T>> {
    self.swap_remove(&node.borrow().get_key())
  }
  pub fn swap_remove(&self, key: &T::Key) -> Option<RcHash<T>> {
    self.inner_mut().swap_remove(key)
  }

  pub fn shift_remove_by_node(&self, node: &RcHash<T>) -> Option<RcHash<T>> {
    self.shift_remove(&node.borrow().get_key())
  }
  pub fn shift_remove(&self, key: &T::Key) -> Option<RcHash<T>> {
    self.inner_mut().shift_remove(key)
  }

  pub fn values(&self) -> IntoValues<T::Key, RcHash<T>> {
    self.inner().clone().into_values()
  }
  pub fn keys(&self) -> IntoKeys<T::Key, RcHash<T>> {
    self.inner().clone().into_keys()
  }
}

impl<T: BseMapNode> IntoIterator for BseMap<T> {
  type Item = (T::Key, RcHash<T>);
  type IntoIter = IntoIter<T::Key, RcHash<T>>;

  fn into_iter(self) -> Self::IntoIter {
    self.inner().clone().into_iter()
  }
}

impl<T: BseMapNode> Clone for BseMap<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<T: BseMapNode> Default for BseMap<T> {
  fn default() -> Self {
    Self(RcHash::new(IndexMap::new()))
  }
}

impl<T> Extend<RcHash<T>> for BseMap<T>
where
  T: BseMapNode,
{
  fn extend<I: IntoIterator<Item = RcHash<T>>>(&mut self, iter: I) {
    self
      .0
      .borrow_mut()
      .extend(iter.into_iter().map(|v| (v.borrow().get_key(), v.clone())))
  }
}

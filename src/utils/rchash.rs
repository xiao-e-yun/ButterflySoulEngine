use std::{
  cell::{Ref, RefCell, RefMut},
  hash::{Hash, Hasher},
  rc::{Rc, Weak},
};

#[derive(Debug, Default)]
pub struct RcHash<T: ?Sized>(Rc<RefCell<T>>);

impl<T> RcHash<T> {
  pub fn new(value: T) -> RcHash<T> {
    RcHash(Rc::new(RefCell::new(value)))
  }
  #[track_caller]
  pub fn borrow(&self) -> Ref<T> {
    self.0.borrow()
  }
  #[track_caller]
  pub fn borrow_mut(&self) -> RefMut<T> {
    self.0.borrow_mut()
  }
  pub fn as_ptr(&self) -> *const T {
    self.0.as_ptr()
  }
  #[track_caller]
  pub fn downgrade(&self) -> WeakHash<T> {
    WeakHash(Rc::downgrade(&self.0))
  }
}

impl<T> PartialEq for RcHash<T> {
  fn eq(&self, other: &RcHash<T>) -> bool {
    Rc::ptr_eq(&self.0, &other.0)
  }
}

impl<T> Eq for RcHash<T> {}

impl<T> Clone for RcHash<T> {
  fn clone(&self) -> Self {
    RcHash(self.0.clone())
  }
}

impl<T> Hash for RcHash<T> {
  fn hash<H>(&self, hasher: &mut H)
  where
    H: Hasher,
  {
    hasher.write_usize(Rc::as_ptr(&self.0) as usize);
  }
}

impl<T> From<Rc<RefCell<T>>> for RcHash<T> {
  fn from(value: Rc<RefCell<T>>) -> Self {
    RcHash(value)
  }
}

#[derive(Debug)]
pub struct WeakHash<T: ?Sized>(Weak<RefCell<T>>);

impl<T> WeakHash<T> {
  pub fn null() -> Self {
    WeakHash(Weak::new())
  }
  pub fn upgrade(self) -> Option<RcHash<T>> {
    self.0.upgrade().and_then(|value|Some(RcHash(value)))
  }
}

impl<T> PartialEq for WeakHash<T> {
  fn eq(&self, other: &WeakHash<T>) -> bool {
    Weak::ptr_eq(&self.0, &other.0)
  }
}

impl<T> Eq for WeakHash<T> {}

impl<T> Clone for WeakHash<T> {
  fn clone(&self) -> Self {
    WeakHash(self.0.clone())
  }
}

impl<T> Hash for WeakHash<T> {
  fn hash<H>(&self, hasher: &mut H)
  where
    H: Hasher,
  {
    hasher.write_usize(Weak::as_ptr(&self.0) as usize);
  }
}
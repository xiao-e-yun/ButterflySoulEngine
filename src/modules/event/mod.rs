use std::{any::Any, collections::HashMap, fmt::Debug, mem};

use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! define_event {
  ($event: ty) => {
    #[$crate::typetag::serde]
    impl $crate::modules::event::Event for $event {
      fn name(&self) -> String {
        stringify!($event).to_string()
      }
    }
  };
}

#[macro_export]
macro_rules! define_listener {
  ($listener: ty: $event: ty,$callback: expr) => {
    #[$crate::typetag::serde]
    impl $crate::modules::event::EventListener for $listener {
      fn name(&self) -> String {
        stringify!($event).to_string()
      }
      fn callback(&mut self, event: &mut dyn std::any::Any) -> bool {
        let Some(event) = event.downcast_mut::<$event>() else {
          return true;
        };

        //INLINE_CODE
        let result: bool = ($callback)(self, event);
        //INLINE_CODE

        result
      }
    }
  };
}

#[typetag::serde]
pub trait Event {
  fn name(&self) -> String;
}

#[typetag::serde]
pub trait EventListener: Debug {
  fn name(&self) -> String;
  fn callback(&mut self, event: &mut dyn Any) -> bool;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Events {
  inner: HashMap<String, Vec<Box<dyn EventListener>>>,
}

impl Events {
  pub fn new() -> Self {
    Self::default()
  }
  pub fn add(&mut self, event_listener: impl EventListener + 'static) {
    self
      .inner
      .entry(event_listener.name())
      .or_default()
      .push(Box::new(event_listener))
  }
  pub fn trigger(&mut self, event: &mut (impl Event + 'static)) {
    let event_listeners = self.inner.get_mut(&event.name());
    let Some(event_listeners) = event_listeners else {
      return;
    };

    let mut temp = Vec::with_capacity(event_listeners.capacity());
    mem::swap(event_listeners, &mut temp);
    for mut listener in temp {
      let alive = listener.callback(event as &mut dyn Any);
      if alive {
        event_listeners.push(listener)
      };
    }
  }
}

impl Default for Events {
  fn default() -> Self {
    Self {
      inner: HashMap::default(),
    }
  }
}

impl Clone for Events {
  fn clone(&self) -> Self {
    //Serialize and then Deserialize
    let mut inner = HashMap::with_capacity(self.inner.capacity());
    for (name, event_listeners) in self.inner.iter() {
      let mut cloned_event_listeners = Vec::with_capacity(event_listeners.capacity());
      for boxed in event_listeners.iter() {
        let bytes = postcard::to_stdvec(boxed).unwrap();
        let boxed: Box<dyn EventListener> = postcard::from_bytes(bytes.as_slice()).unwrap();
        cloned_event_listeners.push(boxed)
      }
      inner.insert(name.clone(), cloned_event_listeners);
    }

    Self { inner }
  }
}

//
//
//
#[test]
fn test() {
  #[derive(Debug, Clone, Serialize, Deserialize)]
  struct HoverListener(u32);
  define_listener!(HoverListener: MouseEvent,|this: &mut HoverListener,event: &mut MouseEvent|{
    assert!(this.0==100,"listener body is changed");
    assert!(event.0==0.0&&0.0==event.1,"event body is changed");
    true
  });

  #[derive(Debug, Clone, Serialize, Deserialize)]
  struct MouseEvent(f32, f32);
  define_event!(MouseEvent);

  let mut events = Events::new();
  events.add(HoverListener(100));
  events.trigger(&mut MouseEvent(0.0, 0.0));
  panic!("{:?}",serde_json::to_string(&events).unwrap());
}

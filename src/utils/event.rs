use std::{any::Any, collections::HashMap, fmt::Debug};

//==============================================================================================
//==============================================================================================
#[derive(Debug)]
pub struct Events(HashMap<String, Vec<Box<dyn Event>>>);

impl Events {
  pub fn new() -> Events {
    Events(HashMap::new())
  }
  pub fn add(&mut self, event: impl Event + 'static) {
    self
      .0
      .entry(event.name())
      .or_default()
      .push(Box::new(event))
  }
  pub fn trigger<T: EventArg + 'static>(&mut self, arg: T::Arg) {
    if let Some(events) = self.0.get_mut(&T::name()) {
      for event in events.iter_mut() {
        event.trigger(&arg);
      }
    }
  }
}

///
/// Quick write event arg.
/// (Events; DefinedEvent: ArgValue,...)
/// ex.
/// ```
/// trigger_events!(events; HotdogEventArg("arg"));
/// trigger_events!(events; HotdogEventArg("arg"): is_trigger());
/// trigger_events!(events; HotdogEventArg("arg"),Hotdog2EventArg(1623));
/// ```
#[macro_export]
macro_rules! trigger_events {
  ($events:expr; $($event:ident($arg:expr)),+) => {
    $($events.trigger::<$event>($arg))+
  };
  ($events:expr; $($event:ident($arg:expr): $trigger: expr),+) => {
    $(if $trigger { trigger_events!($events;$event($arg)) };)+
  };
}

//==============================================================================================
// Event Arg
//==============================================================================================
pub trait EventArg {
  type Arg: Clone;
  fn name() -> String;
  //utils
  // fn to_bytes(input: Self::Arg) -> Vec<u8> {
  //   postcard::to_allocvec(&input).unwrap()
  // }
  // fn parse(input: &Vec<u8>) -> Self::Arg {
  //   postcard::from_bytes(&input).unwrap()
  // }
}

///
/// Quick write event arg.
/// (EventName, DefinedEvent: ArgType)
/// ex.
/// ```
/// create_event_arg!("hotdog", HotdogEventArg: u32);
/// ```
///
#[macro_export]
macro_rules! create_event_arg {
  ($name:expr, $event:ident: $arg:ty) => {
    pub struct $event;
    impl $crate::utils::event::EventArg for $event {
      fn name() -> String {
        String::from($name)
      }
      type Arg = $arg;
    }
  };
}

///
/// Quick write event arg.
/// (EventName, DefinedEvent: ArgType)
/// ex.
/// ```
/// create_event!(Hotdog1Event: HotdogEventArg,|this,e|{});
/// ```
///
#[macro_export]
macro_rules! defined_event {
  ($event:ty: $arg:ident, $callback:expr) => {
    impl $crate::utils::event::Event for $event {
      fn name(&self) -> String {
        use $crate::utils::event::EventArg;
        $arg::name()
      }
      fn trigger(&mut self, arg: &dyn std::any::Any) -> bool {
        let this: &mut $event = self;
        let result = $callback(
          this,
          arg
            .downcast_ref::<<$arg as $crate::utils::event::EventArg>::Arg>()
            .unwrap(),
        );
        result
      }
    }
  };
}

//==============================================================================================
// Event
//==============================================================================================

/// ```rust
///
/// pub struct HotdogEventArg;
/// impl EventArg for HotdogEventArg {
///   type Arg = u32;
///   fn name() -> String { String::new() }
/// }
/// ```
/// if return `false` then remove it
pub trait Event: Debug {
  fn name(&self) -> String;
  fn trigger(&mut self, arg: &dyn Any) -> bool;
}

//==============================================================================================
// Test
//==============================================================================================

mod test {
    use serde::{Deserialize, Serialize};

  create_event_arg!("cold",ColdEventArg: String);
  create_event_arg!("hot",HotEventArg: u32);

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct ColdEvent {
    a: u8,
  }

  defined_event!(ColdEvent: ColdEventArg,|this: &mut Self, arg|{
    assert_eq!(arg,"cold");
    assert_eq!(this.a,0);
    false
  });

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct HotEvent;
  defined_event!(HotEvent: HotEventArg,|_,_|{
    assert!(false);
    false
  });

  #[test]
  fn main() {
    let mut events = crate::utils::event::Events::new();
    events.add(ColdEvent { a: 0 });
    events.add(HotEvent);
    trigger_events!(events;ColdEventArg(String::from("cold")));
    trigger_events!(events;HotEventArg(123): false);
  }
}

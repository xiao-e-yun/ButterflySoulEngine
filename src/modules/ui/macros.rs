#[macro_export]
macro_rules! object_method { 
  ($name:ident:$value: ident) => {
    crate::paste! {
      impl UIObject {
        pub fn [<$name:snake>](self) -> Option<$value> {
          if let UIObject::$name([<$name:snake>]) = self {
            Some([<$name:snake>])
          } else {
            None
          }
        }
      }
      impl $value {
        pub fn to_object(self) -> UIObject {
          UIObject::$name(self)
        }
      }
    }
  };
}

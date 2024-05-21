use std::{collections::BTreeMap, sync::{LazyLock, Mutex}};

pub static CONSOLE: LazyLock<Mutex<(BTreeMap<String, String>, Vec<String>)>> =
  LazyLock::new(|| Mutex::new((BTreeMap::new(), vec![])));

#[doc(hidden)]
#[macro_export]
macro_rules! path {
  () => {
    format!("{}:{}", file!(), line!())
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! console {
  () => {
    $crate::modules::console::CONSOLE.lock().unwrap()
  };
}

#[macro_export]
macro_rules! console_info {
  ($($t:tt)*) => {
    $crate::console!().0.insert($crate::path!(),format!($($t)*))
  }
}

#[macro_export]
macro_rules! console_log {
  ($($t:tt)*) => {
    $crate::console!().1.push(format!("[{}] {}",$crate::path!(),format_args!($($t)*)))
  }
}

#[macro_export]
macro_rules! console_flush {
  () => {
    {
      let console = $crate::console!();
      format!(
        "=Info=========================\n{}\n=Log==========================\n{}",
        console.0.values().cloned().collect::<Vec<String>>().join("\n"),
        console.1.join("\n")
      )
    }
  }
}

#[macro_export]
macro_rules! console_clear {
  () => {
    {
      let mut console = $crate::console!();
      console.0.clear();
      console.1.clear();
    }
  };
}

#[test]
fn test() {
  console_info!("hello {}", 1);
  console_log!("hello {}", 1);

  println!("{}", console_flush!());
  console_clear!();
  println!("{}", console_flush!());
}

//! Prints the events of a connected SpaceMouse until Enter is pressed.
//!
//! Uses `SpaceMouseThread`, which scans for the device on a background thread
//! and reconnects automatically.

use spacemouse::SpaceMouseThread;
use std::io;

fn main() {
  let _spacemouse = SpaceMouseThread::new(|event| println!("{event:?}"));
  println!("Listening for SpaceMouse events. Press Enter to quit.");
  let _ = io::stdin().read_line(&mut String::new());
}

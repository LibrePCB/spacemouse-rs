# SpaceMouse Driver

Cross-platform (Windows/macOS/Linux) driver for 3Dconnexion SpaceMouse and
compatible 3D-mouse devices. Accesses the device by raw HID capture
(through the [`hidapi`](https://docs.rs/hidapi) crate), thus doesn't require
any additional software or driver to be installed. In particular, neither
3DxWare nor `spacenavd` is needed.

The crate was built for [LibrePCB](https://librepcb.org)'s SpaceMouse
support, but usable by any application.

## Example

```rs
use spacemouse::{SpaceMouseEvent, SpaceMouse};
use std::time::Duration;

let mut mouse = SpaceMouse::new()?;

// Poll for the device, e.g. every second, until it is found.
while !mouse.connect() {
  std::thread::sleep(Duration::from_secs(1));
}

// Read events until the device is lost (e.g. unplugged).
loop {
  match mouse.read(Duration::from_millis(200))? {
    Some(SpaceMouseEvent::Translation { x, y, z }) => {
      println!("translation: {x} {y} {z}");
    }
    Some(SpaceMouseEvent::Rotation { x, y, z }) => {
      println!("rotation: {x} {y} {z}");
    }
    _ => {}
  }
}
```

In addition to this synchronous API, this crate also offers a threaded API which
handles all the high-level logic and just calls your callback on any event. See
[`SpaceMouseThread`](https://docs.rs/spacemouse/struct.SpaceMouseThread.html)
for details.

Ready-to-run programs are available in the `examples` directory:

```sh
cargo run --example synchronous
cargo run --example threaded
```

## Documentation

Available at [docs.rs](https://docs.rs/spacemouse/).

## Why `hidapi`

This crate depends on the [`hidapi`](https://docs.rs/hidapi) crate for
cross-platform HID access (`hidraw` on Linux, `IOHIDManager` on macOS,
`hid.dll` on Windows).

On Linux, the pure-Rust `hidraw` backend of `hidapi` is used (the
`linux-native-basic-udev` feature), so building and running this crate needs
neither `libudev` nor `libusb`; no `-dev` packages have to be installed. On
macOS and Windows, `hidapi` vendors a small amount of C source code (the
underlying `hidapi` C library) and compiles it as part of `cargo build`, via
the `cc` crate.

## Linux Device Permissions

On Linux, `hidraw` device nodes are readable by root only by default. To
allow accessing a SpaceMouse as a normal user, install the provided udev
rule [`70-spacemouse.rules`](https://github.com/LibrePCB/spacemouse-rs/blob/main/udev/70-spacemouse.rules):

```sh
sudo cp udev/70-spacemouse.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules && sudo udevadm trigger
```

Then unplug and replug the device (or reboot) for the rule to take effect.

## Status & Limitations

This library is in an early stage of development. The API is still subject
to change, and only a small set of real devices has been tested so far.
Feedback and bug reports are welcome.

**Known limitations:**

* Only the classic USB HID devices are supported. Newer wireless models may
  not work.
* Only translation and rotation events are reported. Button and other HID
  reports are silently ignored.
* Only the first compatible device found is used — there is no way to
  select a specific device yet.
* The `hidapi` feature flags are hardcoded in this crate for now. For more
  control over the used HID access method, they would need to be exposed as
  crate features.

## License

Library is licensed under MIT license, see
[LICENSE](https://github.com/LibrePCB/spacemouse-rs/blob/main/LICENSE) for
details.

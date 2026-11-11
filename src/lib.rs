//! SpaceMouse Driver
//!
//! Cross-platform (Windows/macOS/Linux) driver for 3Dconnexion SpaceMouse and
//! compatible 3D-mouse devices. Accesses the device by raw HID capture
//! (through the [`hidapi`](https://docs.rs/hidapi) crate), thus doesn't require
//! any additional software or driver to be installed. In particular, neither
//! 3DxWare nor `spacenavd` is needed.
//!
//! The crate offers two layers:
//!
//! * [`SpaceMouse`] gives synchronous access to a device: scan for and open it,
//!   read events, and set its LED. The caller drives it.
//! * [`SpaceMouseThread`] uses a [`SpaceMouse`] in a background thread (with
//!   auto-reconnect logic) and delivers [`SpaceMouseEvent`]s to a callback.
//!
//! This crate deliberately knows nothing about sensitivity, inversion,
//! calibration or dead-zones. It always reports the device's raw, unscaled axis
//! readings. Data manipulation (scaling, filtering, mapping to application
//! actions, etc.) is left entirely to the caller.
//!
//! # Example
//!
//! See usage examples in [`SpaceMouse`] and [`SpaceMouseThread`].
//!
//! # Linux Device Permissions
//!
//! On Linux, `hidraw` device nodes are readable by root only by default. See
//! [`README.md`](https://github.com/LibrePCB/spacemouse-rs/blob/main/README.md#linux-device-permissions)
//! for instructions to allow non-root access by installing the provided udev
//! rule.

// Fail on warnings if feature "fail-on-warnings" is enabled.
#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

mod error;
mod event;
mod report;
mod spacemouse;
mod thread;

// Re-Exports
pub use error::SpaceMouseError;
pub use event::SpaceMouseEvent;
pub use spacemouse::SpaceMouse;
pub use thread::SpaceMouseThread;

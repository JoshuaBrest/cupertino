#[link(name = "AppKit", kind = "framework")]
extern "C" {}

pub mod nsapplication;
pub use nsapplication::{NSApplication, NSApplicationActivationPolicy};

pub mod nsmenu;
pub use nsmenu::NSMenu;

pub mod nsmenuitem;
pub use nsmenuitem::NSMenuItem;

pub mod nswindow;
pub use nswindow::{NSWindow, NSWindowStyleMask};
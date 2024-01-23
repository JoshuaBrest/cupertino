//! A wrapper for NSApplicaton

use objc2::ffi::NSInteger;
use objc2::rc::Id;
use objc2::runtime::{Bool, NSObject};

// use crate::core::object_nil;

use crate::core::object_nil;

use super::nsmenu::NSMenu;

/// The Options for how to activate the application
pub enum NSApplicationActivationPolicy {
    /// The application is an ordinary app that appears in the Dock and may have a user interface.
    Regular,
    /// The application does not appear in the Dock and does not have a menu bar, but it may be activated programmatically or by clicking on one of its windows.
    Accessory,
    /// The application does not appear in the Dock and may not create windows or be activated.
    Prohibited,
}

/// A struct representing a NSApplication
pub struct NSApplication(Id<NSObject>);

impl NSApplication {
    /// Create a new NSApplication
    pub fn new() -> NSApplication {
        let nsapplication = unsafe { msg_send_id![class!(NSApplication), sharedApplication] };
        NSApplication(nsapplication)
    }

    /// Get the menu bar
    pub fn main_menu(&self) -> Option<NSMenu> {
        let main_menu: Option<Id<NSObject>> = unsafe { msg_send_id![&self.0, mainMenu] };
        Some(NSMenu::from(main_menu?))
    }

    /// Set the menu bar
    pub fn set_main_menu(&self, menu: NSMenu) {
        let _: () = unsafe { msg_send![&self.0, setMainMenu:menu.as_ref().as_ref()] };
    }

    /// Set the activation policy for the application
    pub fn set_activation_policy(&self, policy: NSApplicationActivationPolicy) {
        let policy = match policy {
            NSApplicationActivationPolicy::Regular => 0,
            NSApplicationActivationPolicy::Accessory => 1,
            NSApplicationActivationPolicy::Prohibited => 2,
        } as NSInteger;
        let _: bool = unsafe { msg_send![&self.0, setActivationPolicy:policy] };
    }

    /// Shows the about panel
    pub fn order_front_standard_about_panel(&self) {
        let _: () = unsafe { msg_send![&self.0, orderFrontStandardAboutPanel:object_nil()] };
    }

    /// Activate the application
    pub fn activate<T>(&self, ignoring_other_apps: T)
    where
        T: Into<Bool>,
    {
        let _: () =
            unsafe { msg_send![&self.0, activateIgnoringOtherApps:ignoring_other_apps.into()] };
    }

    /// Run the application
    pub fn run(&self) {
        let _: () = unsafe { msg_send![&self.0, run] };
    }

    /// Check if the application is running
    pub fn is_running(&self) -> bool {
        let is_running: bool = unsafe { msg_send![&self.0, isRunning] };
        is_running
    }

    /// Stop the application
    pub fn stop(&self) {
        let _: () = unsafe { msg_send![&self.0, stop:0 as u64] };
    }

    /// Get the reference
    #[inline(always)]
    pub fn as_ref(&self) -> &Id<NSObject> {
        &self.0
    }
}

impl From<Id<NSObject>> for NSApplication {
    fn from(ptr: Id<NSObject>) -> Self {
        NSApplication(ptr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nsapplication() {
        let app = NSApplication::new();
        app.set_activation_policy(NSApplicationActivationPolicy::Regular);
        app.activate(false);

        std::thread::spawn(|| {
            let app = NSApplication::new();
            app.run();
        });

        app.stop();
    }
}

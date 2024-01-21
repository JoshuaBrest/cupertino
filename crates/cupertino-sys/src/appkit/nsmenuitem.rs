//! A wrapper for NSMenuItem

use objc2::rc::Id;
use objc2::runtime::{NSObject, Sel};

use crate::core::sel_nil;
use crate::foundation::nsstring::NSString;

use super::NSMenu;

/// A struct representing a NSMenuItem
pub struct NSMenuItem(Id<NSObject>);

impl NSMenuItem {
    /// Create a new NSMenuItem
    pub fn new<T>(title: T) -> NSMenuItem
    where
        T: Into<NSString>,
    {
        let menu_item = unsafe { msg_send_id![class!(NSMenuItem), alloc] };
        let menu_item = unsafe {
            msg_send_id![
                menu_item,
                initWithTitle:title.into().as_ref().as_ref()
            ]
        };
        NSMenuItem(menu_item)
    }

    /// Set a submenu
    pub fn set_submenu(&self, submenu: NSMenu) {
        let _: () = unsafe { msg_send![&self.0, setSubmenu:submenu.as_ref().as_ref()] };
    }

    /// Set the action
    pub fn set_action(&self, action: Sel) {
        let _: () = unsafe { msg_send![&self.0, setAction:action] };
    }

    /// Get the reference
    #[inline(always)]
    pub fn as_ref(&self) -> &Id<NSObject> {
        &self.0
    }
}

impl From<Id<NSObject>> for NSMenuItem {
    fn from(ptr: Id<NSObject>) -> Self {
        NSMenuItem(ptr)
    }
}

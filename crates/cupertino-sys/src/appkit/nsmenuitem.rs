//! A wrapper for NSMenuItem


use objc2::runtime::NSObject;
use objc2::rc::Id;

use crate::core::sel_nil;
use crate::foundation::nsstring::NSString;

use super::NSMenu;

/// A struct representing an NSMenuItem
pub struct NSMenuItem(Id<NSObject>);


impl NSMenuItem {
    /// Create a new NSMenuItem
    pub fn new<T>(title: T) -> NSMenuItem where T: Into<NSString> {

        let menu_item = unsafe { msg_send_id![class!(NSMenuItem), alloc] };
        let menu_item = unsafe {
            msg_send_id![
                menu_item,
                initWithTitle:title.into().as_ref().as_ref()
                action:sel_nil()
                keyEquivalent:NSString::new("").as_ref().as_ref()
            ]
        };
        NSMenuItem(menu_item)
    }

    /// Set a submenu
    pub fn set_submenu(&self, submenu: NSMenu) {
        let _: () = unsafe { msg_send![&self.0, setSubmenu:submenu.as_ref().as_ref()] };
    }

    /// Get the reference
    pub fn as_ref(&self) -> &Id<NSObject> {
        &self.0
    }
}

impl From<Id<NSObject>> for NSMenuItem {
    fn from(nsstring: Id<NSObject>) -> Self {
        NSMenuItem(nsstring)
    }
}
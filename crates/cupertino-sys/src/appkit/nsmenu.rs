//! A wrapper for NSMenu

use objc2::runtime::NSObject;
use objc2::rc::Id;

use crate::foundation::nsstring::NSString;

use super::NSMenuItem;

/// A struct representing an NSMenu
pub struct NSMenu(Id<NSObject>);

impl NSMenu {
    /// Create a new NSMenu
    pub fn new<T>(title: T) -> NSMenu where T: Into<NSString> {
        let menu = unsafe { msg_send_id![class!(NSMenu), alloc] };
        let menu = unsafe { msg_send_id![menu, initWithTitle:title.into().as_ref().as_ref()] };
        NSMenu(menu)
    }

    /// Add an item to the menu
    pub fn add_item(&self, item: &NSMenuItem) {
        let _: () = unsafe { msg_send![&self.0, addItem:item.as_ref().as_ref()] };
    }

    /// Get the reference
    pub fn as_ref(&self) -> &Id<NSObject> {
        &self.0
    }
}


impl From<Id<NSObject>> for NSMenu {
    fn from(nsstring: Id<NSObject>) -> Self {
        NSMenu(nsstring)
    }
}
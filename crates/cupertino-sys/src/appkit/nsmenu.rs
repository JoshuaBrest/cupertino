//! A wrapper for NSMenu

use objc2::rc::Id;
use objc2::runtime::NSObject;

use crate::foundation::nsstring::NSString;

use super::NSMenuItem;

/// A struct representing a NSMenu
pub struct NSMenu(Id<NSObject>);

impl NSMenu {
    /// Create a new NSMenu
    pub fn new() -> Self {
        let menu = unsafe { msg_send_id![class!(NSMenu), alloc] };
        let menu = unsafe { msg_send_id![menu, init] };
        NSMenu(menu)
    }

    /// Set the title of the menu
    pub fn add_title(&self, title: &NSString) {
        let _: () = unsafe { msg_send![&self.0, setTitle:title.as_ref().as_ref()] };
    }

    /// Add an item to the menu
    pub fn add_item(&self, item: &NSMenuItem) {
        let _: () = unsafe { msg_send![&self.0, addItem:item.as_ref().as_ref()] };
    }

    /// Get the reference
    #[inline(always)]
    pub fn as_ref(&self) -> &Id<NSObject> {
        &self.0
    }
}

impl From<Id<NSObject>> for NSMenu {
    fn from(ptr: Id<NSObject>) -> Self {
        NSMenu(ptr)
    }
}

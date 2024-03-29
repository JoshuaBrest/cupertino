//! Wrapper for NSButton

use objc2::{rc::Id, runtime::NSObject};

use crate::{core::CGRect, foundation::NSString};

use super::{NSViewLike, NSLayoutGuideLike};

/// A struct representing a NSButton
pub struct NSButton(Id<NSObject>);

impl NSButton {
    /// Create a new NSButton
    pub fn new(frame: CGRect) -> NSButton {
        let button = unsafe { msg_send_id![class!(NSButton), alloc] };
        let button = unsafe { msg_send_id![button, initWithFrame:frame] };
        NSButton(button)
    }

    /// Set the title
    pub fn set_title(&self, title: &NSString) {
        let _: () = unsafe { msg_send![&self.0, setTitle:title.as_ref().as_ref()] };
    }

    /// Set the action
    pub fn set_action(&self, _action: impl Fn() -> ()) {
        todo!()
    }
}

impl NSViewLike for NSButton {
    fn as_ref(&self) -> &Id<NSObject> {
        &self.0
    }
}

impl NSLayoutGuideLike for NSButton {
    fn as_ref(&self) -> &Id<NSObject> {
        &self.0
    }
}

impl From<Id<NSObject>> for NSButton {
    fn from(ptr: Id<NSObject>) -> Self {
        NSButton(ptr)
    }
}

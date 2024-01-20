//! A wrapper for NSTextField

use objc2::rc::Id;
use objc2::runtime::NSObject;

use crate::core::CGRect;
use crate::foundation::nsstring::NSString;

use super::{NSFont, NSViewLike};

/// A struct representing an NSTextField
pub struct NSTextField(Id<NSObject>);

impl NSTextField {
    /// Create a new NSMenuItem
    pub fn new(frame: CGRect) -> NSTextField {
        let text_field = unsafe { msg_send_id![class!(NSTextField), alloc] };
        let text_field = unsafe { msg_send_id![text_field, initWithFrame:frame] };
        NSTextField(text_field)
    }

    /// Set selectable
    pub fn set_selectable(&self, selectable: bool) {
        let _: () = unsafe { msg_send![&self.0, setSelectable:selectable] };
    }

    /// Set editable
    pub fn set_editable(&self, editable: bool) {
        let _: () = unsafe { msg_send![&self.0, setEditable:editable] };
    }

    /// Set the placeholder string
    pub fn set_placeholder_string<T>(&self, placeholder_string: T)
    where
        T: Into<NSString>,
    {
        let _: () = unsafe {
            msg_send![&self.0, setPlaceholderString:placeholder_string.into().as_ref().as_ref()]
        };
    }

    /// Set string value
    pub fn set_string_value<T>(&self, string_value: T)
    where
        T: Into<NSString>,
    {
        let _: () =
            unsafe { msg_send![&self.0, setStringValue:string_value.into().as_ref().as_ref()] };
    }

    /// Draws background
    pub fn set_draws_background(&self, draws_background: bool) {
        let _: () = unsafe { msg_send![&self.0, setDrawsBackground:draws_background] };
    }

    /// Set whether the text field is bordered
    pub fn set_bordered(&self, bordered: bool) {
        let _: () = unsafe { msg_send![&self.0, setBordered:bordered] };
    }

    /// Set whether the text field is bezeled
    pub fn set_bezeled(&self, bezeled: bool) {
        let _: () = unsafe { msg_send![&self.0, setBezeled:bezeled] };
    }

    /// Set the font
    pub fn set_font(&self, font: NSFont) {
        let _: () = unsafe { msg_send![&self.0, setFont:font.as_ref().as_ref()] };
    }

    /// Get the reference
    pub fn as_ref(&self) -> &Id<NSObject> {
        &self.0
    }
}

impl From<Id<NSObject>> for NSTextField {
    fn from(nsstring: Id<NSObject>) -> Self {
        NSTextField(nsstring)
    }
}

impl NSViewLike for NSTextField {
    fn as_ref(&self) -> &Id<NSObject> {
        &self.0
    }
}

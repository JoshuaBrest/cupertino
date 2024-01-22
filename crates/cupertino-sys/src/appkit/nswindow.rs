//! A wrapper for NSWIndow

use objc2::ffi::NSUInteger;
use objc2::rc::Id;
use objc2::runtime::NSObject;

use crate::core::cgrect::CGRect;
use crate::core::object_nil;
use crate::foundation::NSString;

use super::NSView;

use bitflags::bitflags;

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct NSWindowStyleMask: NSUInteger {
        const BORDERLESS = 0;
        const TITLED = 1 << 0;
        const CLOSABLE = 1 << 1;
        const MINIATURIZABLE = 1 << 2;
        const RESIZABLE = 1 << 3;
        #[deprecated(note = "Deprecated in macOS 11.0")]
        const TEXTURED_BACKGROUND = 1 << 8;
        const UNIFIED_TITLE_AND_TOOLBAR = 1 << 12;
        const FULLSCREEN = 1 << 14;
        const FULLSIZE_CONTENT_VIEW = 1 << 15;
        const UTILITY_WINDOW = 1 << 4;
        const DOC_MODAL_WINDOW = 1 << 6;
        const NONACTIVATING_PANEL = 1 << 7;
        const HUD_WINDOW = 1 << 13;
    }
}

/// A struct representing a NSWindow
pub struct NSWindow(Id<NSObject>);

impl NSWindow {
    pub fn new(area: CGRect, style: NSWindowStyleMask) -> NSWindow {
        let style: NSUInteger = style.bits();

        let window = unsafe { msg_send_id![class!(NSWindow), alloc] };
        let window = unsafe {
            msg_send_id![
                window,
                initWithContentRect:area
                styleMask:style
                backing:2 as u64
                defer:false
            ]
        };
        NSWindow(window)
    }

    pub fn set_title<T>(&self, title: T)
    where
        T: Into<NSString>,
    {
        let _: () = unsafe { msg_send![&self.0, setTitle:title.into().as_ref().as_ref()] };
    }

    /// Set the content view
    pub fn set_content_view(&self, view: &NSView) {
        let _: () = unsafe { msg_send![&self.0, setContentView:view.as_ref().as_ref()] };
    }

    /// Makae the window visible
    pub fn make_key_and_order_front(&self) {
        let _: () = unsafe { msg_send![&self.0, makeKeyAndOrderFront:object_nil()] };
    }

    /// Get the reference
    #[inline(always)]
    pub fn as_ref(&self) -> &Id<NSObject> {
        &self.0
    }
}

impl From<Id<NSObject>> for NSWindow {
    fn from(ptr: Id<NSObject>) -> Self {
        NSWindow(ptr)
    }
}

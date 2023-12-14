//! A wrapper for NSWIndow

use objc2::ffi::NSUInteger;
use objc2::runtime::NSObject;
use objc2::rc::Id;

use crate::core::cgrect::CGRect;
use crate::core::object_nil;
use crate::foundation::NSString;

#[repr(usize)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NSWindowStyleMask {
    Borderless = 0,
    Titled = 1 << 0,
    Closable = 1 << 1,
    Miniaturizable = 1 << 2,
    Resizable = 1 << 3,
    #[deprecated(note = "Deprecated in macOS 11.0")]
    TexturedBackground = 1 << 8,
    UnifiedTitleAndToolbar = 1 << 12,
    Fullscreen = 1 << 14,
    FullsizeContentView = 1 << 15,
    UtilityWindow = 1 << 4,
    DocModalWindow = 1 << 6,
    NonactivatingPanel = 1 << 7,
    HudWindow = 1 << 13,
}

/// A struct representing an NSWindow
pub struct NSWindow(Id<NSObject>);

impl NSWindow {
    pub fn new(area: CGRect, style: Vec<NSWindowStyleMask>) -> NSWindow {
        let style: NSUInteger = style.iter().map(|s| *s as usize).sum();
    
        let window = unsafe { msg_send_id![class!(NSWindow), alloc] };
        let window = unsafe {
            msg_send_id![
                window,
                initWithContentRect:area
                styleMask:style
                backing:2 as NSUInteger
                defer:false
            ]
        };
        NSWindow(window)
    }

    pub fn set_title<T>(&self, title: T) where T: Into<NSString> {
        let _: () = unsafe { msg_send![&self.0, setTitle:title.into().as_ref().as_ref()] };
    }

    /// Makae the window visible
    pub fn make_key_and_order_front(&self) {
        let _: () = unsafe { msg_send![&self.0, makeKeyAndOrderFront:object_nil()] };
    }
    
}

impl From<Id<NSObject>> for NSWindow {
    fn from(nsstring: Id<NSObject>) -> Self {
        NSWindow(nsstring)
    }
}
//! A wrapper for NSFont

use objc2::{
    rc::{Allocated, Id},
    runtime::NSObject,
};

use crate::core::CGFloat;

/// A struct representing a NSFont
pub struct NSFont(Id<NSObject>);

#[derive(Copy, Clone, Debug)]
pub enum NSFontWeight {
    UltraLight,
    Thin,
    Light,
    Regular,
    Medium,
    Semibold,
    Bold,
    Heavy,
    Black,
}

impl NSFontWeight {
    pub fn to_cgfloat(&self) -> CGFloat {
        match self {
            &NSFontWeight::UltraLight => -0.8,
            &NSFontWeight::Thin => -0.6,
            &NSFontWeight::Light => -0.4,
            &NSFontWeight::Regular => 0.0,
            &NSFontWeight::Medium => 0.23,
            &NSFontWeight::Semibold => 0.3,
            &NSFontWeight::Bold => 0.4,
            &NSFontWeight::Heavy => 0.56,
            &NSFontWeight::Black => 0.62,
        }
    }
}

impl NSFont {
    /// Create a new NSFont
    pub fn new() -> NSFont {
        let font: Allocated<NSObject> = unsafe { msg_send_id![class!(NSFont), alloc] };
        let font = unsafe { msg_send_id![font, init] };
        NSFont(font)
    }

    /// Create a system font
    pub fn system_font(size: f64, weight: NSFontWeight) -> NSFont {
        let font = unsafe {
            msg_send_id![class!(NSFont), systemFontOfSize:size weight:weight.to_cgfloat()]
        };
        NSFont(font)
    }

    /// Get the reference
    #[inline(always)]
    pub fn as_ref(&self) -> &Id<NSObject> {
        &self.0
    }
}

impl From<Id<NSObject>> for NSFont {
    fn from(ptr: Id<NSObject>) -> Self {
        NSFont(ptr)
    }
}

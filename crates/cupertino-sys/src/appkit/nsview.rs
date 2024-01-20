//! A wrapper for NSView

use objc2::rc::Id;
use objc2::runtime::NSObject;

use crate::core::CGRect;

/// A struct representing a NSView
pub struct NSView(Id<NSObject>);

pub trait NSViewLike {
    fn as_ref(&self) -> &Id<NSObject>;

    #[inline(always)]
    fn add_subview<'a, T>(&self, subview: &T)
    where
        T: NSViewLike,
    {
        let _: () = unsafe { msg_send![self.as_ref(), addSubview:subview.as_ref().as_ref()] };
    }

    #[inline(always)]
    fn superview(&self) -> Option<NSView> {
        let superview: Option<Id<NSObject>> = unsafe { msg_send_id![self.as_ref(), superview] };
        superview.map(|s| NSView(s))
    }

    #[inline(always)]
    fn disable_auto_layout(&self) {
        let _: () =
            unsafe { msg_send![self.as_ref(), setTranslatesAutoresizingMaskIntoConstraints:false] };
    }
}

impl NSView {
    /// Create a new NSView
    pub fn new(frame: CGRect) -> NSView {
        let view = unsafe { msg_send_id![class!(NSView), alloc] };
        let view = unsafe { msg_send_id![view, initWithFrame:frame] };

        NSView(view)
    }

    /// Get the reference
    #[inline(always)]
    pub fn as_ref(&self) -> &Id<NSObject> {
        &self.0
    }
}

impl NSViewLike for NSView {
    fn as_ref(&self) -> &Id<NSObject> {
        &self.0
    }
}

impl From<Id<NSObject>> for NSView {
    fn from(ptr: Id<NSObject>) -> Self {
        NSView(ptr)
    }
}

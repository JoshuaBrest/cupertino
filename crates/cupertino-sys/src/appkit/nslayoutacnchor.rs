//! A wrapper for NSLayoutXAxisAnchor, NSLayoutYAxisAnchor, and NSLayoutAxisAnchor

use objc2::{rc::Id, runtime::NSObject};

use crate::core::CGFloat;

use super::{NSLayoutConstraint, NSViewLike};

/// A trait representing an NSLayoutAnchor
pub trait NSLayoutAnchorLike {
    /// Returns the anchor's ref
    fn as_ref(&self) -> &Id<NSObject>;

    /// Constrains the anchor to a given anchor given another anchor with an offset
    fn anchor(&self, other: &Self, offset: CGFloat) -> NSLayoutConstraint {
        let data: Id<NSObject> = unsafe {
            msg_send_id![
                self.as_ref(),
                constraintEqualToAnchor: other.as_ref().as_ref()
                constant: offset
            ]
        };

        NSLayoutConstraint::from(data)
    }

    /// Constrains the anchor to be greater than or equal to another anchor with an offset
    fn anchor_gt_or_equal(&self, other: &Self, offset: CGFloat) -> NSLayoutConstraint {
        let data: Id<NSObject> = unsafe {
            msg_send_id![
                self.as_ref(),
                constraintGreaterThanOrEqualToAnchor: other.as_ref().as_ref()
                constant: offset
            ]
        };

        NSLayoutConstraint::from(data)
    }

    /// Constrains the anchor to be less than or equal to another anchor with an offset
    fn anchor_lt_or_equal(&self, other: &Self, offset: CGFloat) -> NSLayoutConstraint {
        let data: Id<NSObject> = unsafe {
            msg_send_id![
                self.as_ref(),
                constraintLessThanOrEqualToAnchor: other.as_ref().as_ref()
                constant: offset
            ]
        };

        NSLayoutConstraint::from(data)
    }
}

/// A struct representing an NSLayoutYAxisAnchor
pub struct NSLayoutYAxisAnchor(Id<NSObject>);

impl NSLayoutYAxisAnchor {
    /// Returns a new NSLayoutAxisAnchor (Top)
    pub fn top(view: &impl NSViewLike) -> Self {
        let anchor = unsafe { msg_send_id![view.as_ref(), topAnchor] };
        Self(anchor)
    }

    /// Returns a new NSLayoutAxisAnchor (Bottom)
    pub fn bottom(view: &impl NSViewLike) -> Self {
        let anchor = unsafe { msg_send_id![view.as_ref(), bottomAnchor] };
        Self(anchor)
    }

    /// Returns a new NSLayoutAxisAnchor (CenterY)
    pub fn center_y(view: &impl NSViewLike) -> Self {
        let anchor = unsafe { msg_send_id![view.as_ref(), centerYAnchor] };
        Self(anchor)
    }
}

impl NSLayoutAnchorLike for NSLayoutYAxisAnchor {
    fn as_ref(&self) -> &Id<NSObject> {
        &self.0
    }
}

impl From<Id<NSObject>> for NSLayoutYAxisAnchor {
    fn from(anchor: Id<NSObject>) -> Self {
        Self(anchor)
    }
}

/// A struct representing an NSLayoutXAxisAnchor
pub struct NSLayoutXAxisAnchor(Id<NSObject>);

impl NSLayoutXAxisAnchor {
    /// Returns a new NSLayoutAxisAnchor (Left)
    pub fn left(view: &impl NSViewLike) -> Self {
        let anchor = unsafe { msg_send_id![view.as_ref(), leftAnchor] };
        Self(anchor)
    }

    /// Returns a new NSLayoutAxisAnchor (Right)
    pub fn right(view: &impl NSViewLike) -> Self {
        let anchor = unsafe { msg_send_id![view.as_ref(), rightAnchor] };
        Self(anchor)
    }

    /// Returns a new NSLayoutAxisAnchor (CenterX)
    pub fn center_x(view: &impl NSViewLike) -> Self {
        let anchor = unsafe { msg_send_id![view.as_ref(), centerXAnchor] };
        Self(anchor)
    }
}

impl NSLayoutAnchorLike for NSLayoutXAxisAnchor {
    fn as_ref(&self) -> &Id<NSObject> {
        &self.0
    }
}

impl From<Id<NSObject>> for NSLayoutXAxisAnchor {
    fn from(anchor: Id<NSObject>) -> Self {
        Self(anchor)
    }
}

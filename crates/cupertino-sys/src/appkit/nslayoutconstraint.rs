//! A wrapper for NSLayoutConstraint.

use objc2::{rc::Id, runtime::NSObject};

use crate::{core::CGFloat, foundation::NSArray};

/// A struct representing a NSLayoutConstraint.
pub struct NSLayoutConstraint(Id<NSObject>);

impl NSLayoutConstraint {
    /// Activate the constraint.
    pub fn activate(&self) {
        let _: () = unsafe { msg_send![self.0.as_ref(), setActive:true] };
    }

    /// Deactivate the constraint.
    pub fn deactivate(&self) {
        let _: () = unsafe { msg_send![self.0.as_ref(), setActive:false] };
    }

    /// Set the priority of the constraint.
    pub fn set_priority(&self, priority: CGFloat) {
        let _: () = unsafe { msg_send![self.0.as_ref(), setPriority:priority] };
    }

    /// Set the constant (offset) of the constraint.
    pub fn set_constant(&self, constant: CGFloat) {
        let _: () = unsafe { msg_send![self.0.as_ref(), setConstant:constant] };
    }

    /// Set the multiplier of the constraint.
    pub fn set_multiplier(&self, multiplier: CGFloat) {
        let _: () = unsafe { msg_send![self.0.as_ref(), setMultiplier:multiplier] };
    }

    /// Activate multiple constraints at once.
    pub fn activate_constraints(constraints: &[NSLayoutConstraint]) {
        // Convert constraints to list of NSLayoutConstraint pointers.
        let constraints = constraints.iter().map(|c| c.0.as_ref()).collect::<Vec<_>>();
        let constraints = NSArray::new(constraints);

        // Activate constraints.
        let _: () = unsafe {
            msg_send![class!(NSLayoutConstraint), activateConstraints:constraints.as_ref().as_ref()]
        };
    }

    /// Deactivate multiple constraints at once.
    pub fn deactivate_constraints(constraints: &[NSLayoutConstraint]) {
        // Convert constraints to list of NSLayoutConstraint pointers.
        let constraints = constraints.iter().map(|c| c.0.as_ref()).collect::<Vec<_>>();
        let constraints = NSArray::new(constraints);

        // Deactivate constraints.
        let _: () = unsafe {
            msg_send![class!(NSLayoutConstraint), deactivateConstraints:constraints.as_ref().as_ref()]
        };
    }
}

impl From<Id<NSObject>> for NSLayoutConstraint {
    fn from(id: Id<NSObject>) -> Self {
        NSLayoutConstraint(id)
    }
}

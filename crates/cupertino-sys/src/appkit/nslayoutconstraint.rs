//! A wrapper for NSLayoutConstraint.

use objc2::{rc::Id, runtime::NSObject, Encode};

use crate::foundation::NSArray;

/// A wrapper for NSLayoutConstraint.
pub struct NSLayoutConstraint(Id<NSObject>);

impl NSLayoutConstraint {
    /// Activate multiple constraints at once.
    pub fn activate_constraints(constraints: &[NSLayoutConstraint]) {
        // Convert constraints to list of NSLayoutConstraint pointers.
        let constraints = constraints.iter().map(|c| c.0.as_ref().as_ref()).collect::<Vec<_>>();
        let constraints = NSArray::new(constraints);

        println!("{:?}", constraints.as_ref());

        // Activate constraints.
        let _: () =
            unsafe { msg_send![class!(NSLayoutConstraint), activateConstraints:constraints.as_ref().as_ref()]
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

//! Binding for NSArray

use objc2::{rc::Id, runtime::NSObject};

pub struct NSArray(Id<NSObject>);

impl NSArray {
    pub fn new(objects: Vec<&NSObject>) -> Self {
        let array = unsafe {
            msg_send_id![
                class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count: objects.len()
            ]
        };

        NSArray(array)
    }

    pub fn count(&self) -> usize {
        let count: usize = unsafe { msg_send![&self.0, count] };
        count
    }

    #[inline(always)]
    pub fn as_ref(&self) -> &Id<NSObject> {
        &self.0
    }
}

impl From<Id<NSObject>> for NSArray {
    fn from(id: Id<NSObject>) -> Self {
        NSArray(id)
    }
}

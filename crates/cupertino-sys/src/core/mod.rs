#[cfg(target_pointer_width = "32")]
type CGFloat = f32;
#[cfg(target_pointer_width = "64")]
type CGFloat = f64;



pub mod cgpoint;
pub use cgpoint::CGPoint;

pub mod cgsize;
pub use cgsize::CGSize;

pub mod cgrect;
pub use cgrect::CGRect;

pub mod nil;
pub use nil::{sel_nil, object_nil};
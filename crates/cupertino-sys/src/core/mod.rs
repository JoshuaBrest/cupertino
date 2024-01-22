#[cfg(target_pointer_width = "32")]
pub type CGFloat = f32;
#[cfg(target_pointer_width = "64")]
pub type CGFloat = f64;

pub mod cgpoint;
pub use cgpoint::CGPoint;

pub mod cgsize;
pub use cgsize::CGSize;

pub mod cgrect;
pub use cgrect::CGRect;

pub mod nil;
pub use nil::object_nil;

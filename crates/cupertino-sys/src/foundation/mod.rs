#[link(name = "Foundation", kind = "framework")]
extern "C" {}

pub mod nsstring;
pub use nsstring::NSString;

pub mod nsarray;
pub use nsarray::NSArray;

//! C-Style struct for CGSize

use objc2::{Encode, Encoding};

use crate::core::CGFloat;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

unsafe impl Encode for CGSize {
    const ENCODING: Encoding = Encoding::Struct(
        "CGSize",
        &[
            CGFloat::ENCODING,
            CGFloat::ENCODING,
        ]
    );
}
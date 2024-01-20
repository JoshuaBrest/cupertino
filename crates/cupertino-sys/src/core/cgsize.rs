//! C-Style struct for CGSize

use objc2::{Encode, Encoding};

use crate::core::CGFloat;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

impl CGSize {
    pub fn new(width: f64, height: f64) -> Self {
        CGSize { width, height }
    }

    pub fn zero() -> Self {
        CGSize {
            width: 0.0,
            height: 0.0,
        }
    }
}

unsafe impl Encode for CGSize {
    const ENCODING: Encoding = Encoding::Struct("CGSize", &[CGFloat::ENCODING, CGFloat::ENCODING]);
}

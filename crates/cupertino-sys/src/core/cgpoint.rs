//! C-Style struct for CGPoint

use objc2::{Encode, Encoding};

use crate::core::CGFloat;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}

impl CGPoint {
    pub fn new(x: f64, y: f64) -> Self {
        CGPoint { x, y }
    }

    pub fn zero() -> Self {
        CGPoint { x: 0.0, y: 0.0 }
    }
}

unsafe impl Encode for CGPoint {
    const ENCODING: Encoding = Encoding::Struct("CGPoint", &[CGFloat::ENCODING, CGFloat::ENCODING]);
}

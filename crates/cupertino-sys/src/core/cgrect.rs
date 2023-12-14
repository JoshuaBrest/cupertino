//! C-Style struct for CGRect

use objc2::{Encode, Encoding};

use crate::core::cgpoint::CGPoint;
use crate::core::cgsize::CGSize;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
}

impl CGRect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        CGRect {
            origin: CGPoint { x, y },
            size: CGSize { width, height },
        }
    }
}

unsafe impl Encode for CGRect {
    const ENCODING: Encoding = Encoding::Struct(
        "CGRect",
        &[
            CGPoint::ENCODING,
            CGSize::ENCODING,
        ]
    );
}
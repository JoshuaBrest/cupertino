use objc2::{Encode, Encoding};

#[derive(Copy, Clone, Debug, PartialEq)]
struct OpaqueData {
    _private: [u8; 0],
}

#[repr(transparent)]
pub struct Sel {
    _ptr: OpaqueData
}

unsafe impl Encode for Sel {
    const ENCODING: Encoding = Encoding::Sel;
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NSObject {
    _ptr: OpaqueData
}

unsafe impl Encode for NSObject {
    const ENCODING: Encoding = Encoding::Object;
}

pub fn sel_nil() -> Sel {
    // Create an OpaqueData from a pointer to a null pointer
    let ptr = std::ptr::null::<*const u8>();
    let ptr = ptr as *const OpaqueData;

    // Create a Sel from the OpaqueData
    unsafe { Sel { _ptr: *ptr } }
}

pub fn object_nil() -> NSObject {
    // Create an OpaqueData from a pointer to a null pointer
    let ptr = std::ptr::null::<*const u8>();
    let ptr = ptr as *const OpaqueData;

    // Create a Sel from the OpaqueData
    unsafe { NSObject { _ptr: *ptr } }
}
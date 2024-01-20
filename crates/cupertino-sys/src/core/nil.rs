use objc2::{Encode, Encoding};

#[derive(Copy, Clone, Debug, PartialEq)]
struct OpaqueData {
    _private: [u8; 0],
}

#[doc(hidden)]
#[repr(transparent)]
pub struct DontUseNilSel {
    _ptr: OpaqueData,
}

unsafe impl Encode for DontUseNilSel {
    const ENCODING: Encoding = Encoding::Sel;
}

#[doc(hidden)]
#[repr(transparent)]
pub struct DontUseObjectSel {
    _ptr: OpaqueData,
}

unsafe impl Encode for DontUseObjectSel {
    const ENCODING: Encoding = Encoding::Object;
}

pub fn sel_nil() -> DontUseNilSel {
    // Create an OpaqueData from a pointer to a null pointer
    let ptr = std::ptr::null::<*const u8>();
    let ptr = ptr as *const OpaqueData;

    // Create a Sel from the OpaqueData
    unsafe { DontUseNilSel { _ptr: *ptr } }
}

pub fn object_nil() -> DontUseObjectSel {
    // Create an OpaqueData from a pointer to a null pointer
    let ptr = std::ptr::null::<*const u8>();
    let ptr = ptr as *const OpaqueData;

    // Create a Sel from the OpaqueData
    unsafe { DontUseObjectSel { _ptr: *ptr } }
}

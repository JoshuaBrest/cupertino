use objc2::runtime::NSObject;

#[derive(Copy, Clone, Debug, PartialEq)]
struct OpaqueData {
    _private: [u8; 0],
}

pub fn object_nil() -> *mut NSObject {
    let nil: *mut NSObject = unsafe { std::mem::transmute(0 as u64) };

    nil
}

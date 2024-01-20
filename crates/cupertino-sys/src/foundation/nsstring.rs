//! Bindings for NSString
use std::ffi::c_void;
use std::fmt::{Debug, Display};

use objc2::rc::Id;
use objc2::runtime::NSObject;

/// A struct representing an NSString
pub struct NSString(Id<NSObject>);

impl NSString {
    /// Create a new NSString from a Rust string
    pub fn new<T>(string: T) -> NSString
    where
        T: Into<String>,
    {
        let string = string.into();
        let nsstring = NSString::from_str(string);
        nsstring
    }

    /// Create a new NSString from a Rust string slice
    pub fn from_str(string: String) -> NSString {
        let string_ref: *const c_void = string.as_ptr() as *const c_void;
        let nsstring = unsafe { msg_send_id![class!(NSString), alloc] };
        let nsstring = unsafe {
            msg_send_id![nsstring, initWithBytes:string_ref length:string.len() encoding:4 as usize]
        };

        NSString(nsstring)
    }

    /// Convert the NSString to a Rust string
    pub fn to_string(&self) -> String {
        let bytes: *const u8 = unsafe { msg_send![&self.0, UTF8String] };
        let bytes = unsafe { core::slice::from_raw_parts(bytes, self.len()) };
        let string = unsafe { core::str::from_utf8_unchecked(bytes) };
        string.to_owned()
    }

    /// Get the length of the NSString
    pub fn len(&self) -> usize {
        let length: usize = unsafe { msg_send![&self.0, lengthOfBytesUsingEncoding:4 as usize] };
        length
    }

    /// Get the ref
    pub fn as_ref(&self) -> &Id<NSObject> {
        &self.0
    }
}

impl From<Id<NSObject>> for NSString {
    fn from(nsstring: Id<NSObject>) -> Self {
        NSString(nsstring)
    }
}

impl From<NSString> for Id<NSObject> {
    fn from(nsstring: NSString) -> Self {
        nsstring.0
    }
}

impl From<&str> for NSString {
    fn from(string: &str) -> Self {
        NSString::new(string)
    }
}

impl From<String> for NSString {
    fn from(string: String) -> Self {
        NSString::new(string)
    }
}

impl Debug for NSString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string: String = self.to_string();
        write!(f, "NSString({})", string)
    }
}

impl Display for NSString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string: String = self.to_string();
        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii() {
        let nsstring = NSString::new("Hello, world!");
        let string = nsstring.to_string();
        assert_eq!(string, "Hello, world!");
    }

    #[test]
    fn test_unicode() {
        let nsstring = NSString::new("❌, world!");
        let string = nsstring.to_string();
        assert_eq!(string, "❌, world!");
    }

    #[test]
    fn test_empty() {
        let nsstring = NSString::new("");
        let string = nsstring.to_string();
        assert_eq!(string, "");
    }
}

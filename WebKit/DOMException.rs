//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/domexception?language=objc)
    pub static DOMException: Option<&'static NSString>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/webkit/domexceptioncode?language=objc)
#[deprecated]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DOMExceptionCode(pub c_uint);
impl DOMExceptionCode {
    #[deprecated]
    pub const DOM_INDEX_SIZE_ERR: Self = Self(1);
    #[deprecated]
    pub const DOM_DOMSTRING_SIZE_ERR: Self = Self(2);
    #[deprecated]
    pub const DOM_HIERARCHY_REQUEST_ERR: Self = Self(3);
    #[deprecated]
    pub const DOM_WRONG_DOCUMENT_ERR: Self = Self(4);
    #[deprecated]
    pub const DOM_INVALID_CHARACTER_ERR: Self = Self(5);
    #[deprecated]
    pub const DOM_NO_DATA_ALLOWED_ERR: Self = Self(6);
    #[deprecated]
    pub const DOM_NO_MODIFICATION_ALLOWED_ERR: Self = Self(7);
    #[deprecated]
    pub const DOM_NOT_FOUND_ERR: Self = Self(8);
    #[deprecated]
    pub const DOM_NOT_SUPPORTED_ERR: Self = Self(9);
    #[deprecated]
    pub const DOM_INUSE_ATTRIBUTE_ERR: Self = Self(10);
    #[deprecated]
    pub const DOM_INVALID_STATE_ERR: Self = Self(11);
    #[deprecated]
    pub const DOM_SYNTAX_ERR: Self = Self(12);
    #[deprecated]
    pub const DOM_INVALID_MODIFICATION_ERR: Self = Self(13);
    #[deprecated]
    pub const DOM_NAMESPACE_ERR: Self = Self(14);
    #[deprecated]
    pub const DOM_INVALID_ACCESS_ERR: Self = Self(15);
}

unsafe impl Encode for DOMExceptionCode {
    const ENCODING: Encoding = c_uint::ENCODING;
}

unsafe impl RefEncode for DOMExceptionCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

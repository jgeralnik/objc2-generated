//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/identitylookup/ilmessagefiltererrordomain?language=objc)
    pub static ILMessageFilterErrorDomain: &'static NSErrorDomain;
}

/// [Apple's documentation](https://developer.apple.com/documentation/identitylookup/ilmessagefiltererror?language=objc)
// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ILMessageFilterError(pub NSInteger);
impl ILMessageFilterError {
    #[doc(alias = "ILMessageFilterErrorSystem")]
    pub const System: Self = Self(1);
    #[doc(alias = "ILMessageFilterErrorInvalidNetworkURL")]
    pub const InvalidNetworkURL: Self = Self(2);
    #[doc(alias = "ILMessageFilterErrorNetworkURLUnauthorized")]
    pub const NetworkURLUnauthorized: Self = Self(3);
    #[doc(alias = "ILMessageFilterErrorNetworkRequestFailed")]
    pub const NetworkRequestFailed: Self = Self(4);
    #[doc(alias = "ILMessageFilterErrorRedundantNetworkDeferral")]
    pub const RedundantNetworkDeferral: Self = Self(5);
}

unsafe impl Encode for ILMessageFilterError {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for ILMessageFilterError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

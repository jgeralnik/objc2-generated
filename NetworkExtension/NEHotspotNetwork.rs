//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NEHotspotNetworkSecurityType(pub NSInteger);
impl NEHotspotNetworkSecurityType {
    #[doc(alias = "NEHotspotNetworkSecurityTypeOpen")]
    pub const Open: Self = Self(0);
    #[doc(alias = "NEHotspotNetworkSecurityTypeWEP")]
    pub const WEP: Self = Self(1);
    #[doc(alias = "NEHotspotNetworkSecurityTypePersonal")]
    pub const Personal: Self = Self(2);
    #[doc(alias = "NEHotspotNetworkSecurityTypeEnterprise")]
    pub const Enterprise: Self = Self(3);
    #[doc(alias = "NEHotspotNetworkSecurityTypeUnknown")]
    pub const Unknown: Self = Self(4);
}

unsafe impl Encode for NEHotspotNetworkSecurityType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NEHotspotNetworkSecurityType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEHotspotNetwork;

    unsafe impl ClassType for NEHotspotNetwork {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NEHotspotNetwork {}

extern_methods!(
    unsafe impl NEHotspotNetwork {
        #[method_id(@__retain_semantics Other SSID)]
        pub unsafe fn SSID(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Other BSSID)]
        pub unsafe fn BSSID(&self) -> Id<NSString>;

        #[method(securityType)]
        pub unsafe fn securityType(&self) -> NEHotspotNetworkSecurityType;

        #[cfg(feature = "block2")]
        #[method(fetchCurrentWithCompletionHandler:)]
        pub unsafe fn fetchCurrentWithCompletionHandler(
            completion_handler: &block2::Block<dyn Fn(*mut NEHotspotNetwork)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEHotspotNetwork {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

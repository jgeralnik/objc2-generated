//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

extern "C" {
    #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
    pub static ILMessageFilterErrorDomain: &'static NSErrorDomain;
}

ns_error_enum!(
    #[underlying(NSInteger)]
    pub enum ILMessageFilterError {
        #[doc(alias = "ILMessageFilterErrorSystem")]
        System = 1,
        #[doc(alias = "ILMessageFilterErrorInvalidNetworkURL")]
        InvalidNetworkURL = 2,
        #[doc(alias = "ILMessageFilterErrorNetworkURLUnauthorized")]
        NetworkURLUnauthorized = 3,
        #[doc(alias = "ILMessageFilterErrorNetworkRequestFailed")]
        NetworkRequestFailed = 4,
        #[doc(alias = "ILMessageFilterErrorRedundantNetworkDeferral")]
        RedundantNetworkDeferral = 5,
    }
);

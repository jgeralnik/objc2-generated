//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/ascredentialidentity?language=objc)
    pub unsafe trait ASCredentialIdentity: NSObjectProtocol {
        #[cfg(feature = "ASCredentialServiceIdentifier")]
        #[method_id(@__retain_semantics Other serviceIdentifier)]
        unsafe fn serviceIdentifier(&self) -> Retained<ASCredentialServiceIdentifier>;

        #[method_id(@__retain_semantics Other user)]
        unsafe fn user(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other recordIdentifier)]
        unsafe fn recordIdentifier(&self) -> Option<Retained<NSString>>;

        #[method(rank)]
        unsafe fn rank(&self) -> NSInteger;

        #[method(setRank:)]
        unsafe fn setRank(&self, rank: NSInteger);
    }

    unsafe impl ProtocolType for dyn ASCredentialIdentity {}
);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ASAuthorizationRequest")]
    pub struct ASAuthorizationPasswordRequest;

    #[cfg(feature = "ASAuthorizationRequest")]
    unsafe impl ClassType for ASAuthorizationPasswordRequest {
        #[inherits(NSObject)]
        type Super = ASAuthorizationRequest;
    }
);

#[cfg(feature = "ASAuthorizationRequest")]
unsafe impl NSCoding for ASAuthorizationPasswordRequest {}

#[cfg(feature = "ASAuthorizationRequest")]
unsafe impl NSCopying for ASAuthorizationPasswordRequest {}

#[cfg(feature = "ASAuthorizationRequest")]
unsafe impl CopyingHelper for ASAuthorizationPasswordRequest {
    type Result = Self;
}

#[cfg(feature = "ASAuthorizationRequest")]
unsafe impl NSObjectProtocol for ASAuthorizationPasswordRequest {}

#[cfg(feature = "ASAuthorizationRequest")]
unsafe impl NSSecureCoding for ASAuthorizationPasswordRequest {}

extern_methods!(
    #[cfg(feature = "ASAuthorizationRequest")]
    unsafe impl ASAuthorizationPasswordRequest {}
);

extern_methods!(
    /// Methods declared on superclass `ASAuthorizationRequest`
    #[cfg(feature = "ASAuthorizationRequest")]
    unsafe impl ASAuthorizationPasswordRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

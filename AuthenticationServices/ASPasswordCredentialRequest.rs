//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/aspasswordcredentialrequest?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASPasswordCredentialRequest;
);

#[cfg(feature = "ASCredentialRequest")]
unsafe impl ASCredentialRequest for ASPasswordCredentialRequest {}

unsafe impl NSCoding for ASPasswordCredentialRequest {}

unsafe impl NSCopying for ASPasswordCredentialRequest {}

unsafe impl CopyingHelper for ASPasswordCredentialRequest {
    type Result = Self;
}

unsafe impl NSObjectProtocol for ASPasswordCredentialRequest {}

unsafe impl NSSecureCoding for ASPasswordCredentialRequest {}

extern_methods!(
    unsafe impl ASPasswordCredentialRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "ASPasswordCredentialIdentity")]
        #[method_id(@__retain_semantics Init initWithCredentialIdentity:)]
        pub unsafe fn initWithCredentialIdentity(
            this: Allocated<Self>,
            credential_identity: &ASPasswordCredentialIdentity,
        ) -> Retained<Self>;

        #[cfg(feature = "ASPasswordCredentialIdentity")]
        #[method_id(@__retain_semantics Other requestWithCredentialIdentity:)]
        pub unsafe fn requestWithCredentialIdentity(
            credential_identity: &ASPasswordCredentialIdentity,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASPasswordCredentialRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

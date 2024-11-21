//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASOneTimeCodeCredentialRequest;
);

#[cfg(feature = "ASCredentialRequest")]
unsafe impl ASCredentialRequest for ASOneTimeCodeCredentialRequest {}

unsafe impl NSCoding for ASOneTimeCodeCredentialRequest {}

unsafe impl NSCopying for ASOneTimeCodeCredentialRequest {}

unsafe impl CopyingHelper for ASOneTimeCodeCredentialRequest {
    type Result = Self;
}

unsafe impl NSObjectProtocol for ASOneTimeCodeCredentialRequest {}

unsafe impl NSSecureCoding for ASOneTimeCodeCredentialRequest {}

extern_methods!(
    unsafe impl ASOneTimeCodeCredentialRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "ASOneTimeCodeCredentialIdentity")]
        #[method_id(@__retain_semantics Init initWithCredentialIdentity:)]
        pub unsafe fn initWithCredentialIdentity(
            this: Allocated<Self>,
            credential_identity: &ASOneTimeCodeCredentialIdentity,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASOneTimeCodeCredentialRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

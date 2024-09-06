//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationPublicKeyCredentialParameters;

    unsafe impl ClassType for ASAuthorizationPublicKeyCredentialParameters {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for ASAuthorizationPublicKeyCredentialParameters {}

unsafe impl NSCopying for ASAuthorizationPublicKeyCredentialParameters {}

unsafe impl CopyingHelper for ASAuthorizationPublicKeyCredentialParameters {
    type Result = Self;
}

unsafe impl NSObjectProtocol for ASAuthorizationPublicKeyCredentialParameters {}

unsafe impl NSSecureCoding for ASAuthorizationPublicKeyCredentialParameters {}

extern_methods!(
    unsafe impl ASAuthorizationPublicKeyCredentialParameters {
        #[cfg(feature = "ASCOSEConstants")]
        #[method_id(@__retain_semantics Init initWithAlgorithm:)]
        pub unsafe fn initWithAlgorithm(
            this: Allocated<Self>,
            algorithm: ASCOSEAlgorithmIdentifier,
        ) -> Retained<Self>;

        #[cfg(feature = "ASCOSEConstants")]
        #[method(algorithm)]
        pub unsafe fn algorithm(&self) -> ASCOSEAlgorithmIdentifier;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

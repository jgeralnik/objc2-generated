//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASPasskeyRegistrationCredentialExtensionOutput;

    unsafe impl ClassType for ASPasskeyRegistrationCredentialExtensionOutput {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for ASPasskeyRegistrationCredentialExtensionOutput {}

unsafe impl NSCopying for ASPasskeyRegistrationCredentialExtensionOutput {}

unsafe impl CopyingHelper for ASPasskeyRegistrationCredentialExtensionOutput {
    type Result = Self;
}

unsafe impl NSObjectProtocol for ASPasskeyRegistrationCredentialExtensionOutput {}

unsafe impl NSSecureCoding for ASPasskeyRegistrationCredentialExtensionOutput {}

extern_methods!(
    unsafe impl ASPasskeyRegistrationCredentialExtensionOutput {
        #[cfg(feature = "ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput")]
        #[method_id(@__retain_semantics Init initWithLargeBlobOutput:)]
        pub unsafe fn initWithLargeBlobOutput(
            this: Allocated<Self>,
            large_blob: Option<&ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput>,
        ) -> Retained<Self>;

        #[cfg(feature = "ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput")]
        #[method_id(@__retain_semantics Other largeBlobRegistrationOutput)]
        pub unsafe fn largeBlobRegistrationOutput(
            &self,
        ) -> Option<Retained<ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASPasskeyRegistrationCredentialExtensionOutput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
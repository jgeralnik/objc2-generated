//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationSecurityKeyPublicKeyCredentialRegistration;

    unsafe impl ClassType for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration {
        type Super = NSObject;
    }
);

#[cfg(feature = "ASAuthorizationCredential")]
unsafe impl ASAuthorizationCredential
    for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration
{
}

#[cfg(all(
    feature = "ASAuthorizationCredential",
    feature = "ASAuthorizationPublicKeyCredentialRegistration",
    feature = "ASPublicKeyCredential"
))]
unsafe impl ASAuthorizationPublicKeyCredentialRegistration
    for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration
{
}

#[cfg(all(
    feature = "ASAuthorizationCredential",
    feature = "ASPublicKeyCredential"
))]
unsafe impl ASPublicKeyCredential for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration {}

unsafe impl NSCoding for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration {}

unsafe impl NSCopying for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration {}

unsafe impl CopyingHelper for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration {}

unsafe impl NSSecureCoding for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration {}

extern_methods!(
    unsafe impl ASAuthorizationSecurityKeyPublicKeyCredentialRegistration {
        #[cfg(feature = "ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor")]
        #[method_id(@__retain_semantics Other transports)]
        pub unsafe fn transports(
            &self,
        ) -> Retained<NSArray<ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransport>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASAuthorizationSecurityKeyPublicKeyCredentialRegistration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

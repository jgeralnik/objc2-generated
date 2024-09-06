//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ASAuthorizationRequest")]
    pub struct ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest;

    #[cfg(feature = "ASAuthorizationRequest")]
    unsafe impl ClassType for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest {
        #[inherits(NSObject)]
        type Super = ASAuthorizationRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "ASAuthorizationPublicKeyCredentialRegistrationRequest",
    feature = "ASAuthorizationRequest"
))]
unsafe impl ASAuthorizationPublicKeyCredentialRegistrationRequest
    for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest
{
}

#[cfg(feature = "ASAuthorizationRequest")]
unsafe impl NSCoding for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest {}

#[cfg(feature = "ASAuthorizationRequest")]
unsafe impl NSCopying for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest {}

#[cfg(feature = "ASAuthorizationRequest")]
unsafe impl CopyingHelper for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest {
    type Result = Self;
}

#[cfg(feature = "ASAuthorizationRequest")]
unsafe impl NSObjectProtocol for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest {}

#[cfg(feature = "ASAuthorizationRequest")]
unsafe impl NSSecureCoding for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest {}

extern_methods!(
    #[cfg(feature = "ASAuthorizationRequest")]
    unsafe impl ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "ASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput")]
        #[method_id(@__retain_semantics Other largeBlob)]
        pub unsafe fn largeBlob(
            &self,
        ) -> Option<Retained<ASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput>>;

        #[cfg(feature = "ASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput")]
        #[method(setLargeBlob:)]
        pub unsafe fn setLargeBlob(
            &self,
            large_blob: Option<&ASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput>,
        );
    }
);

extern_methods!(
    #[cfg(feature = "ASAuthorizationRequest")]
    unsafe impl ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest {}
);

#[cfg(all(
    feature = "ASAuthorizationRequest",
    feature = "ASAuthorizationWebBrowserPlatformPublicKeyCredentialRegistrationRequest"
))]
unsafe impl ASAuthorizationWebBrowserPlatformPublicKeyCredentialRegistrationRequest
    for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest
{
}

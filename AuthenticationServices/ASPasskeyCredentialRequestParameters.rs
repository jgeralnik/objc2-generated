//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASPasskeyCredentialRequestParameters;

    unsafe impl ClassType for ASPasskeyCredentialRequestParameters {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for ASPasskeyCredentialRequestParameters {}

unsafe impl Sync for ASPasskeyCredentialRequestParameters {}

unsafe impl NSCoding for ASPasskeyCredentialRequestParameters {}

unsafe impl NSCopying for ASPasskeyCredentialRequestParameters {}

unsafe impl CopyingHelper for ASPasskeyCredentialRequestParameters {
    type Result = Self;
}

unsafe impl NSObjectProtocol for ASPasskeyCredentialRequestParameters {}

unsafe impl NSSecureCoding for ASPasskeyCredentialRequestParameters {}

extern_methods!(
    unsafe impl ASPasskeyCredentialRequestParameters {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other relyingPartyIdentifier)]
        pub unsafe fn relyingPartyIdentifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other clientDataHash)]
        pub unsafe fn clientDataHash(&self) -> Retained<NSData>;

        #[cfg(feature = "ASAuthorizationPublicKeyCredentialConstants")]
        #[method_id(@__retain_semantics Other userVerificationPreference)]
        pub unsafe fn userVerificationPreference(
            &self,
        ) -> Retained<ASAuthorizationPublicKeyCredentialUserVerificationPreference>;

        #[method_id(@__retain_semantics Other allowedCredentials)]
        pub unsafe fn allowedCredentials(&self) -> Retained<NSArray<NSData>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASPasskeyCredentialRequestParameters {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

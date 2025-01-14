//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asonetimecodecredentialidentity?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASOneTimeCodeCredentialIdentity;
);

#[cfg(feature = "ASCredentialIdentity")]
unsafe impl ASCredentialIdentity for ASOneTimeCodeCredentialIdentity {}

unsafe impl NSCoding for ASOneTimeCodeCredentialIdentity {}

unsafe impl NSCopying for ASOneTimeCodeCredentialIdentity {}

unsafe impl CopyingHelper for ASOneTimeCodeCredentialIdentity {
    type Result = Self;
}

unsafe impl NSObjectProtocol for ASOneTimeCodeCredentialIdentity {}

unsafe impl NSSecureCoding for ASOneTimeCodeCredentialIdentity {}

extern_methods!(
    unsafe impl ASOneTimeCodeCredentialIdentity {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "ASCredentialServiceIdentifier")]
        #[method_id(@__retain_semantics Init initWithServiceIdentifier:label:recordIdentifier:)]
        pub unsafe fn initWithServiceIdentifier_label_recordIdentifier(
            this: Allocated<Self>,
            service_identifier: &ASCredentialServiceIdentifier,
            label: &NSString,
            record_identifier: Option<&NSString>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASOneTimeCodeCredentialIdentity {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

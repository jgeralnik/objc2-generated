//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/aspasswordcredentialidentity?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASPasswordCredentialIdentity;
);

#[cfg(feature = "ASCredentialIdentity")]
unsafe impl ASCredentialIdentity for ASPasswordCredentialIdentity {}

unsafe impl NSCoding for ASPasswordCredentialIdentity {}

unsafe impl NSCopying for ASPasswordCredentialIdentity {}

unsafe impl CopyingHelper for ASPasswordCredentialIdentity {
    type Result = Self;
}

unsafe impl NSObjectProtocol for ASPasswordCredentialIdentity {}

unsafe impl NSSecureCoding for ASPasswordCredentialIdentity {}

extern_methods!(
    unsafe impl ASPasswordCredentialIdentity {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "ASCredentialServiceIdentifier")]
        #[method_id(@__retain_semantics Init initWithServiceIdentifier:user:recordIdentifier:)]
        pub unsafe fn initWithServiceIdentifier_user_recordIdentifier(
            this: Allocated<Self>,
            service_identifier: &ASCredentialServiceIdentifier,
            user: &NSString,
            record_identifier: Option<&NSString>,
        ) -> Retained<Self>;

        #[cfg(feature = "ASCredentialServiceIdentifier")]
        #[method_id(@__retain_semantics Other identityWithServiceIdentifier:user:recordIdentifier:)]
        pub unsafe fn identityWithServiceIdentifier_user_recordIdentifier(
            service_identifier: &ASCredentialServiceIdentifier,
            user: &NSString,
            record_identifier: Option<&NSString>,
        ) -> Retained<Self>;

        #[cfg(feature = "ASCredentialServiceIdentifier")]
        #[method_id(@__retain_semantics Other serviceIdentifier)]
        pub unsafe fn serviceIdentifier(&self) -> Retained<ASCredentialServiceIdentifier>;

        #[method_id(@__retain_semantics Other user)]
        pub unsafe fn user(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other recordIdentifier)]
        pub unsafe fn recordIdentifier(&self) -> Option<Retained<NSString>>;

        #[method(rank)]
        pub unsafe fn rank(&self) -> NSInteger;

        #[method(setRank:)]
        pub unsafe fn setRank(&self, rank: NSInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASPasswordCredentialIdentity {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

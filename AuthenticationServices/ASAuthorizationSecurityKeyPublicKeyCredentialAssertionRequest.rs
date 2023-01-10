//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest;

    unsafe impl ClassType for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {
        #[inherits(NSObject)]
        type Super = ASAuthorizationRequest;
    }
);

extern_methods!(
    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest"
    )]
    unsafe impl ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {
        #[cfg(
            feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor"
        )]
        #[method_id(@__retain_semantics Other allowedCredentials)]
        pub unsafe fn allowedCredentials(
            &self,
        ) -> Id<NSArray<ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor>, Shared>;

        #[cfg(
            feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor"
        )]
        #[method(setAllowedCredentials:)]
        pub unsafe fn setAllowedCredentials(
            &self,
            allowedCredentials: &NSArray<ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor>,
        );
    }
);

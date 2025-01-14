//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationwebbrowserplatformpublickeycredential?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationWebBrowserPlatformPublicKeyCredential;
);

unsafe impl NSObjectProtocol for ASAuthorizationWebBrowserPlatformPublicKeyCredential {}

extern_methods!(
    unsafe impl ASAuthorizationWebBrowserPlatformPublicKeyCredential {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other customTitle)]
        pub unsafe fn customTitle(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other relyingParty)]
        pub unsafe fn relyingParty(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other credentialID)]
        pub unsafe fn credentialID(&self) -> Retained<NSData>;

        #[method_id(@__retain_semantics Other userHandle)]
        pub unsafe fn userHandle(&self) -> Retained<NSData>;

        #[method_id(@__retain_semantics Other providerName)]
        pub unsafe fn providerName(&self) -> Retained<NSString>;
    }
);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationappleidrequest?language=objc)
    #[unsafe(super(ASAuthorizationOpenIDRequest, ASAuthorizationRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "ASAuthorizationOpenIDRequest",
        feature = "ASAuthorizationRequest"
    ))]
    pub struct ASAuthorizationAppleIDRequest;
);

#[cfg(all(
    feature = "ASAuthorizationOpenIDRequest",
    feature = "ASAuthorizationRequest"
))]
unsafe impl NSCoding for ASAuthorizationAppleIDRequest {}

#[cfg(all(
    feature = "ASAuthorizationOpenIDRequest",
    feature = "ASAuthorizationRequest"
))]
unsafe impl NSCopying for ASAuthorizationAppleIDRequest {}

#[cfg(all(
    feature = "ASAuthorizationOpenIDRequest",
    feature = "ASAuthorizationRequest"
))]
unsafe impl CopyingHelper for ASAuthorizationAppleIDRequest {
    type Result = Self;
}

#[cfg(all(
    feature = "ASAuthorizationOpenIDRequest",
    feature = "ASAuthorizationRequest"
))]
unsafe impl NSObjectProtocol for ASAuthorizationAppleIDRequest {}

#[cfg(all(
    feature = "ASAuthorizationOpenIDRequest",
    feature = "ASAuthorizationRequest"
))]
unsafe impl NSSecureCoding for ASAuthorizationAppleIDRequest {}

extern_methods!(
    #[cfg(all(
        feature = "ASAuthorizationOpenIDRequest",
        feature = "ASAuthorizationRequest"
    ))]
    unsafe impl ASAuthorizationAppleIDRequest {
        #[method_id(@__retain_semantics Other user)]
        pub unsafe fn user(&self) -> Option<Retained<NSString>>;

        #[method(setUser:)]
        pub unsafe fn setUser(&self, user: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `ASAuthorizationRequest`
    #[cfg(all(
        feature = "ASAuthorizationOpenIDRequest",
        feature = "ASAuthorizationRequest"
    ))]
    unsafe impl ASAuthorizationAppleIDRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

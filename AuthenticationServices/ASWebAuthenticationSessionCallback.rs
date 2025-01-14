//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/aswebauthenticationsessioncallback?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASWebAuthenticationSessionCallback;
);

unsafe impl Send for ASWebAuthenticationSessionCallback {}

unsafe impl Sync for ASWebAuthenticationSessionCallback {}

unsafe impl NSObjectProtocol for ASWebAuthenticationSessionCallback {}

extern_methods!(
    unsafe impl ASWebAuthenticationSessionCallback {
        #[method_id(@__retain_semantics Other callbackWithCustomScheme:)]
        pub unsafe fn callbackWithCustomScheme(custom_scheme: &NSString) -> Retained<Self>;

        #[method_id(@__retain_semantics Other callbackWithHTTPSHost:path:)]
        pub unsafe fn callbackWithHTTPSHost_path(
            host: &NSString,
            path: &NSString,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method(matchesURL:)]
        pub unsafe fn matchesURL(&self, url: &NSURL) -> bool;
    }
);

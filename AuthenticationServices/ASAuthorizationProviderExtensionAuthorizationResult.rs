//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationProviderExtensionAuthorizationResult;
);

unsafe impl NSObjectProtocol for ASAuthorizationProviderExtensionAuthorizationResult {}

extern_methods!(
    unsafe impl ASAuthorizationProviderExtensionAuthorizationResult {
        #[method_id(@__retain_semantics Init initWithHTTPAuthorizationHeaders:)]
        pub unsafe fn initWithHTTPAuthorizationHeaders(
            this: Allocated<Self>,
            http_authorization_headers: &NSDictionary<NSString, NSString>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithHTTPResponse:httpBody:)]
        pub unsafe fn initWithHTTPResponse_httpBody(
            this: Allocated<Self>,
            http_response: &NSHTTPURLResponse,
            http_body: Option<&NSData>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other httpAuthorizationHeaders)]
        pub unsafe fn httpAuthorizationHeaders(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, NSString>>>;

        #[method(setHttpAuthorizationHeaders:)]
        pub unsafe fn setHttpAuthorizationHeaders(
            &self,
            http_authorization_headers: Option<&NSDictionary<NSString, NSString>>,
        );

        #[method_id(@__retain_semantics Other httpResponse)]
        pub unsafe fn httpResponse(&self) -> Option<Retained<NSHTTPURLResponse>>;

        #[method(setHttpResponse:)]
        pub unsafe fn setHttpResponse(&self, http_response: Option<&NSHTTPURLResponse>);

        #[method_id(@__retain_semantics Other httpBody)]
        pub unsafe fn httpBody(&self) -> Option<Retained<NSData>>;

        #[method(setHttpBody:)]
        pub unsafe fn setHttpBody(&self, http_body: Option<&NSData>);

        #[method_id(@__retain_semantics Other privateKeys)]
        pub unsafe fn privateKeys(&self) -> Retained<NSArray>;

        #[method(setPrivateKeys:)]
        pub unsafe fn setPrivateKeys(&self, private_keys: &NSArray);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASAuthorizationProviderExtensionAuthorizationResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

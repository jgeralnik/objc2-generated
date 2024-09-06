//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSURLProtectionSpaceHTTP: &'static NSString;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSURLProtectionSpaceHTTPS: &'static NSString;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSURLProtectionSpaceFTP: &'static NSString;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSURLProtectionSpaceHTTPProxy: &'static NSString;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSURLProtectionSpaceHTTPSProxy: &'static NSString;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSURLProtectionSpaceFTPProxy: &'static NSString;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSURLProtectionSpaceSOCKSProxy: &'static NSString;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSURLAuthenticationMethodDefault: &'static NSString;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSURLAuthenticationMethodHTTPBasic: &'static NSString;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSURLAuthenticationMethodHTTPDigest: &'static NSString;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSURLAuthenticationMethodHTMLForm: &'static NSString;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSURLAuthenticationMethodNTLM: &'static NSString;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSURLAuthenticationMethodNegotiate: &'static NSString;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSURLAuthenticationMethodClientCertificate: &'static NSString;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSURLAuthenticationMethodServerTrust: &'static NSString;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSURLProtectionSpace;

    unsafe impl ClassType for NSURLProtectionSpace {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for NSURLProtectionSpace {}

unsafe impl Sync for NSURLProtectionSpace {}

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSURLProtectionSpace {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSURLProtectionSpace {}

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSURLProtectionSpace {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSURLProtectionSpace {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSURLProtectionSpace {}

extern_methods!(
    unsafe impl NSURLProtectionSpace {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Init initWithHost:port:protocol:realm:authenticationMethod:)]
        pub unsafe fn initWithHost_port_protocol_realm_authenticationMethod(
            this: Allocated<Self>,
            host: &NSString,
            port: NSInteger,
            protocol: Option<&NSString>,
            realm: Option<&NSString>,
            authentication_method: Option<&NSString>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Init initWithProxyHost:port:type:realm:authenticationMethod:)]
        pub unsafe fn initWithProxyHost_port_type_realm_authenticationMethod(
            this: Allocated<Self>,
            host: &NSString,
            port: NSInteger,
            r#type: Option<&NSString>,
            realm: Option<&NSString>,
            authentication_method: Option<&NSString>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other realm)]
        pub unsafe fn realm(&self) -> Option<Retained<NSString>>;

        #[method(receivesCredentialSecurely)]
        pub unsafe fn receivesCredentialSecurely(&self) -> bool;

        #[method(isProxy)]
        pub unsafe fn isProxy(&self) -> bool;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other host)]
        pub unsafe fn host(&self) -> Retained<NSString>;

        #[method(port)]
        pub unsafe fn port(&self) -> NSInteger;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other proxyType)]
        pub unsafe fn proxyType(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other protocol)]
        pub unsafe fn protocol(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other authenticationMethod)]
        pub unsafe fn authenticationMethod(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSURLProtectionSpace {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSClientCertificateSpace
    unsafe impl NSURLProtectionSpace {
        #[cfg(all(feature = "NSArray", feature = "NSData"))]
        #[method_id(@__retain_semantics Other distinguishedNames)]
        pub unsafe fn distinguishedNames(&self) -> Option<Retained<NSArray<NSData>>>;
    }
);

extern_methods!(
    /// NSServerTrustValidationSpace
    unsafe impl NSURLProtectionSpace {}
);

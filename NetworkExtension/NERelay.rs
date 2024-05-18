//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NERelay;

    unsafe impl ClassType for NERelay {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for NERelay {}

unsafe impl NSCopying for NERelay {}

unsafe impl NSObjectProtocol for NERelay {}

unsafe impl NSSecureCoding for NERelay {}

extern_methods!(
    unsafe impl NERelay {
        #[method_id(@__retain_semantics Other HTTP3RelayURL)]
        pub unsafe fn HTTP3RelayURL(&self) -> Option<Id<NSURL>>;

        #[method(setHTTP3RelayURL:)]
        pub unsafe fn setHTTP3RelayURL(&self, http3_relay_url: Option<&NSURL>);

        #[method_id(@__retain_semantics Other HTTP2RelayURL)]
        pub unsafe fn HTTP2RelayURL(&self) -> Option<Id<NSURL>>;

        #[method(setHTTP2RelayURL:)]
        pub unsafe fn setHTTP2RelayURL(&self, http2_relay_url: Option<&NSURL>);

        #[method_id(@__retain_semantics Other dnsOverHTTPSURL)]
        pub unsafe fn dnsOverHTTPSURL(&self) -> Option<Id<NSURL>>;

        #[method(setDnsOverHTTPSURL:)]
        pub unsafe fn setDnsOverHTTPSURL(&self, dns_over_httpsurl: Option<&NSURL>);

        #[method_id(@__retain_semantics Other syntheticDNSAnswerIPv4Prefix)]
        pub unsafe fn syntheticDNSAnswerIPv4Prefix(&self) -> Option<Id<NSString>>;

        #[method(setSyntheticDNSAnswerIPv4Prefix:)]
        pub unsafe fn setSyntheticDNSAnswerIPv4Prefix(
            &self,
            synthetic_dns_answer_i_pv4_prefix: Option<&NSString>,
        );

        #[method_id(@__retain_semantics Other syntheticDNSAnswerIPv6Prefix)]
        pub unsafe fn syntheticDNSAnswerIPv6Prefix(&self) -> Option<Id<NSString>>;

        #[method(setSyntheticDNSAnswerIPv6Prefix:)]
        pub unsafe fn setSyntheticDNSAnswerIPv6Prefix(
            &self,
            synthetic_dns_answer_i_pv6_prefix: Option<&NSString>,
        );

        #[method_id(@__retain_semantics Other additionalHTTPHeaderFields)]
        pub unsafe fn additionalHTTPHeaderFields(&self) -> Id<NSDictionary<NSString, NSString>>;

        #[method(setAdditionalHTTPHeaderFields:)]
        pub unsafe fn setAdditionalHTTPHeaderFields(
            &self,
            additional_http_header_fields: &NSDictionary<NSString, NSString>,
        );

        #[method_id(@__retain_semantics Other rawPublicKeys)]
        pub unsafe fn rawPublicKeys(&self) -> Option<Id<NSArray<NSData>>>;

        #[method(setRawPublicKeys:)]
        pub unsafe fn setRawPublicKeys(&self, raw_public_keys: Option<&NSArray<NSData>>);

        #[method_id(@__retain_semantics Other identityData)]
        pub unsafe fn identityData(&self) -> Option<Id<NSData>>;

        #[method(setIdentityData:)]
        pub unsafe fn setIdentityData(&self, identity_data: Option<&NSData>);

        #[method_id(@__retain_semantics Other identityDataPassword)]
        pub unsafe fn identityDataPassword(&self) -> Option<Id<NSString>>;

        #[method(setIdentityDataPassword:)]
        pub unsafe fn setIdentityDataPassword(&self, identity_data_password: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NERelay {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

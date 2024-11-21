//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NEVPNProtocol, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NEVPNProtocol")]
    pub struct NEDNSProxyProviderProtocol;
);

#[cfg(feature = "NEVPNProtocol")]
unsafe impl NSCoding for NEDNSProxyProviderProtocol {}

#[cfg(feature = "NEVPNProtocol")]
unsafe impl NSCopying for NEDNSProxyProviderProtocol {}

#[cfg(feature = "NEVPNProtocol")]
unsafe impl CopyingHelper for NEDNSProxyProviderProtocol {
    type Result = Self;
}

#[cfg(feature = "NEVPNProtocol")]
unsafe impl NSObjectProtocol for NEDNSProxyProviderProtocol {}

#[cfg(feature = "NEVPNProtocol")]
unsafe impl NSSecureCoding for NEDNSProxyProviderProtocol {}

extern_methods!(
    #[cfg(feature = "NEVPNProtocol")]
    unsafe impl NEDNSProxyProviderProtocol {
        #[method_id(@__retain_semantics Other providerConfiguration)]
        pub unsafe fn providerConfiguration(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[method(setProviderConfiguration:)]
        pub unsafe fn setProviderConfiguration(
            &self,
            provider_configuration: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[method_id(@__retain_semantics Other providerBundleIdentifier)]
        pub unsafe fn providerBundleIdentifier(&self) -> Option<Retained<NSString>>;

        #[method(setProviderBundleIdentifier:)]
        pub unsafe fn setProviderBundleIdentifier(
            &self,
            provider_bundle_identifier: Option<&NSString>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NEVPNProtocol")]
    unsafe impl NEDNSProxyProviderProtocol {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

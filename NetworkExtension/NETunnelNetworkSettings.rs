//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NETunnelNetworkSettings;

    unsafe impl ClassType for NETunnelNetworkSettings {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for NETunnelNetworkSettings {}

unsafe impl NSCopying for NETunnelNetworkSettings {}

unsafe impl CopyingHelper for NETunnelNetworkSettings {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NETunnelNetworkSettings {}

unsafe impl NSSecureCoding for NETunnelNetworkSettings {}

extern_methods!(
    unsafe impl NETunnelNetworkSettings {
        #[method_id(@__retain_semantics Init initWithTunnelRemoteAddress:)]
        pub unsafe fn initWithTunnelRemoteAddress(
            this: Allocated<Self>,
            address: &NSString,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other tunnelRemoteAddress)]
        pub unsafe fn tunnelRemoteAddress(&self) -> Retained<NSString>;

        #[cfg(feature = "NEDNSSettings")]
        #[method_id(@__retain_semantics Other DNSSettings)]
        pub unsafe fn DNSSettings(&self) -> Option<Retained<NEDNSSettings>>;

        #[cfg(feature = "NEDNSSettings")]
        #[method(setDNSSettings:)]
        pub unsafe fn setDNSSettings(&self, dns_settings: Option<&NEDNSSettings>);

        #[cfg(feature = "NEProxySettings")]
        #[method_id(@__retain_semantics Other proxySettings)]
        pub unsafe fn proxySettings(&self) -> Option<Retained<NEProxySettings>>;

        #[cfg(feature = "NEProxySettings")]
        #[method(setProxySettings:)]
        pub unsafe fn setProxySettings(&self, proxy_settings: Option<&NEProxySettings>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NETunnelNetworkSettings {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

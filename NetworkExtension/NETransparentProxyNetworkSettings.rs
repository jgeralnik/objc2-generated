//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NETunnelNetworkSettings")]
    pub struct NETransparentProxyNetworkSettings;

    #[cfg(feature = "NETunnelNetworkSettings")]
    unsafe impl ClassType for NETransparentProxyNetworkSettings {
        #[inherits(NSObject)]
        type Super = NETunnelNetworkSettings;
    }
);

#[cfg(feature = "NETunnelNetworkSettings")]
unsafe impl NSCoding for NETransparentProxyNetworkSettings {}

#[cfg(feature = "NETunnelNetworkSettings")]
unsafe impl NSCopying for NETransparentProxyNetworkSettings {}

#[cfg(feature = "NETunnelNetworkSettings")]
unsafe impl CopyingHelper for NETransparentProxyNetworkSettings {
    type Result = Self;
}

#[cfg(feature = "NETunnelNetworkSettings")]
unsafe impl NSObjectProtocol for NETransparentProxyNetworkSettings {}

#[cfg(feature = "NETunnelNetworkSettings")]
unsafe impl NSSecureCoding for NETransparentProxyNetworkSettings {}

extern_methods!(
    #[cfg(feature = "NETunnelNetworkSettings")]
    unsafe impl NETransparentProxyNetworkSettings {
        #[cfg(feature = "NENetworkRule")]
        #[method_id(@__retain_semantics Other includedNetworkRules)]
        pub unsafe fn includedNetworkRules(&self) -> Option<Retained<NSArray<NENetworkRule>>>;

        #[cfg(feature = "NENetworkRule")]
        #[method(setIncludedNetworkRules:)]
        pub unsafe fn setIncludedNetworkRules(
            &self,
            included_network_rules: Option<&NSArray<NENetworkRule>>,
        );

        #[cfg(feature = "NENetworkRule")]
        #[method_id(@__retain_semantics Other excludedNetworkRules)]
        pub unsafe fn excludedNetworkRules(&self) -> Option<Retained<NSArray<NENetworkRule>>>;

        #[cfg(feature = "NENetworkRule")]
        #[method(setExcludedNetworkRules:)]
        pub unsafe fn setExcludedNetworkRules(
            &self,
            excluded_network_rules: Option<&NSArray<NENetworkRule>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NETunnelNetworkSettings`
    #[cfg(feature = "NETunnelNetworkSettings")]
    unsafe impl NETransparentProxyNetworkSettings {
        #[method_id(@__retain_semantics Init initWithTunnelRemoteAddress:)]
        pub unsafe fn initWithTunnelRemoteAddress(
            this: Allocated<Self>,
            address: &NSString,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NETunnelNetworkSettings")]
    unsafe impl NETransparentProxyNetworkSettings {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

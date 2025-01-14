//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/networkextension/neipv6settings?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEIPv6Settings;
);

unsafe impl NSCoding for NEIPv6Settings {}

unsafe impl NSCopying for NEIPv6Settings {}

unsafe impl CopyingHelper for NEIPv6Settings {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NEIPv6Settings {}

unsafe impl NSSecureCoding for NEIPv6Settings {}

extern_methods!(
    unsafe impl NEIPv6Settings {
        #[method_id(@__retain_semantics Init initWithAddresses:networkPrefixLengths:)]
        pub unsafe fn initWithAddresses_networkPrefixLengths(
            this: Allocated<Self>,
            addresses: &NSArray<NSString>,
            network_prefix_lengths: &NSArray<NSNumber>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other settingsWithAutomaticAddressing)]
        pub unsafe fn settingsWithAutomaticAddressing() -> Retained<Self>;

        #[method_id(@__retain_semantics Other settingsWithLinkLocalAddressing)]
        pub unsafe fn settingsWithLinkLocalAddressing() -> Retained<Self>;

        #[method_id(@__retain_semantics Other addresses)]
        pub unsafe fn addresses(&self) -> Retained<NSArray<NSString>>;

        #[method_id(@__retain_semantics Other networkPrefixLengths)]
        pub unsafe fn networkPrefixLengths(&self) -> Retained<NSArray<NSNumber>>;

        #[method_id(@__retain_semantics Other includedRoutes)]
        pub unsafe fn includedRoutes(&self) -> Option<Retained<NSArray<NEIPv6Route>>>;

        #[method(setIncludedRoutes:)]
        pub unsafe fn setIncludedRoutes(&self, included_routes: Option<&NSArray<NEIPv6Route>>);

        #[method_id(@__retain_semantics Other excludedRoutes)]
        pub unsafe fn excludedRoutes(&self) -> Option<Retained<NSArray<NEIPv6Route>>>;

        #[method(setExcludedRoutes:)]
        pub unsafe fn setExcludedRoutes(&self, excluded_routes: Option<&NSArray<NEIPv6Route>>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEIPv6Settings {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/networkextension/neipv6route?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEIPv6Route;
);

unsafe impl NSCoding for NEIPv6Route {}

unsafe impl NSCopying for NEIPv6Route {}

unsafe impl CopyingHelper for NEIPv6Route {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NEIPv6Route {}

unsafe impl NSSecureCoding for NEIPv6Route {}

extern_methods!(
    unsafe impl NEIPv6Route {
        #[method_id(@__retain_semantics Init initWithDestinationAddress:networkPrefixLength:)]
        pub unsafe fn initWithDestinationAddress_networkPrefixLength(
            this: Allocated<Self>,
            address: &NSString,
            network_prefix_length: &NSNumber,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other destinationAddress)]
        pub unsafe fn destinationAddress(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other destinationNetworkPrefixLength)]
        pub unsafe fn destinationNetworkPrefixLength(&self) -> Retained<NSNumber>;

        #[method_id(@__retain_semantics Other gatewayAddress)]
        pub unsafe fn gatewayAddress(&self) -> Option<Retained<NSString>>;

        #[method(setGatewayAddress:)]
        pub unsafe fn setGatewayAddress(&self, gateway_address: Option<&NSString>);

        #[method_id(@__retain_semantics Other defaultRoute)]
        pub unsafe fn defaultRoute() -> Retained<NEIPv6Route>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEIPv6Route {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

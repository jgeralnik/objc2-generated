//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/networkextension/neipv4settings?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEIPv4Settings;
);

unsafe impl NSCoding for NEIPv4Settings {}

unsafe impl NSCopying for NEIPv4Settings {}

unsafe impl CopyingHelper for NEIPv4Settings {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NEIPv4Settings {}

unsafe impl NSSecureCoding for NEIPv4Settings {}

extern_methods!(
    unsafe impl NEIPv4Settings {
        #[method_id(@__retain_semantics Init initWithAddresses:subnetMasks:)]
        pub unsafe fn initWithAddresses_subnetMasks(
            this: Allocated<Self>,
            addresses: &NSArray<NSString>,
            subnet_masks: &NSArray<NSString>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other settingsWithAutomaticAddressing)]
        pub unsafe fn settingsWithAutomaticAddressing() -> Retained<Self>;

        #[method_id(@__retain_semantics Other addresses)]
        pub unsafe fn addresses(&self) -> Retained<NSArray<NSString>>;

        #[method_id(@__retain_semantics Other subnetMasks)]
        pub unsafe fn subnetMasks(&self) -> Retained<NSArray<NSString>>;

        #[method_id(@__retain_semantics Other router)]
        pub unsafe fn router(&self) -> Option<Retained<NSString>>;

        #[method(setRouter:)]
        pub unsafe fn setRouter(&self, router: Option<&NSString>);

        #[method_id(@__retain_semantics Other includedRoutes)]
        pub unsafe fn includedRoutes(&self) -> Option<Retained<NSArray<NEIPv4Route>>>;

        #[method(setIncludedRoutes:)]
        pub unsafe fn setIncludedRoutes(&self, included_routes: Option<&NSArray<NEIPv4Route>>);

        #[method_id(@__retain_semantics Other excludedRoutes)]
        pub unsafe fn excludedRoutes(&self) -> Option<Retained<NSArray<NEIPv4Route>>>;

        #[method(setExcludedRoutes:)]
        pub unsafe fn setExcludedRoutes(&self, excluded_routes: Option<&NSArray<NEIPv4Route>>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEIPv4Settings {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/networkextension/neipv4route?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEIPv4Route;
);

unsafe impl NSCoding for NEIPv4Route {}

unsafe impl NSCopying for NEIPv4Route {}

unsafe impl CopyingHelper for NEIPv4Route {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NEIPv4Route {}

unsafe impl NSSecureCoding for NEIPv4Route {}

extern_methods!(
    unsafe impl NEIPv4Route {
        #[method_id(@__retain_semantics Init initWithDestinationAddress:subnetMask:)]
        pub unsafe fn initWithDestinationAddress_subnetMask(
            this: Allocated<Self>,
            address: &NSString,
            subnet_mask: &NSString,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other destinationAddress)]
        pub unsafe fn destinationAddress(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other destinationSubnetMask)]
        pub unsafe fn destinationSubnetMask(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other gatewayAddress)]
        pub unsafe fn gatewayAddress(&self) -> Option<Retained<NSString>>;

        #[method(setGatewayAddress:)]
        pub unsafe fn setGatewayAddress(&self, gateway_address: Option<&NSString>);

        #[method_id(@__retain_semantics Other defaultRoute)]
        pub unsafe fn defaultRoute() -> Retained<NEIPv4Route>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEIPv4Route {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

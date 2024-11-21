//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEFilterProviderConfiguration;
);

unsafe impl NSCoding for NEFilterProviderConfiguration {}

unsafe impl NSCopying for NEFilterProviderConfiguration {}

unsafe impl CopyingHelper for NEFilterProviderConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NEFilterProviderConfiguration {}

unsafe impl NSSecureCoding for NEFilterProviderConfiguration {}

extern_methods!(
    unsafe impl NEFilterProviderConfiguration {
        #[deprecated = "filterBrowsers is not supported on macOS"]
        #[method(filterBrowsers)]
        pub unsafe fn filterBrowsers(&self) -> bool;

        #[deprecated = "filterBrowsers is not supported on macOS"]
        #[method(setFilterBrowsers:)]
        pub unsafe fn setFilterBrowsers(&self, filter_browsers: bool);

        #[method(filterSockets)]
        pub unsafe fn filterSockets(&self) -> bool;

        #[method(setFilterSockets:)]
        pub unsafe fn setFilterSockets(&self, filter_sockets: bool);

        #[method(filterPackets)]
        pub unsafe fn filterPackets(&self) -> bool;

        #[method(setFilterPackets:)]
        pub unsafe fn setFilterPackets(&self, filter_packets: bool);

        #[method_id(@__retain_semantics Other vendorConfiguration)]
        pub unsafe fn vendorConfiguration(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[method(setVendorConfiguration:)]
        pub unsafe fn setVendorConfiguration(
            &self,
            vendor_configuration: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[method_id(@__retain_semantics Other serverAddress)]
        pub unsafe fn serverAddress(&self) -> Option<Retained<NSString>>;

        #[method(setServerAddress:)]
        pub unsafe fn setServerAddress(&self, server_address: Option<&NSString>);

        #[method_id(@__retain_semantics Other username)]
        pub unsafe fn username(&self) -> Option<Retained<NSString>>;

        #[method(setUsername:)]
        pub unsafe fn setUsername(&self, username: Option<&NSString>);

        #[method_id(@__retain_semantics Other organization)]
        pub unsafe fn organization(&self) -> Option<Retained<NSString>>;

        #[method(setOrganization:)]
        pub unsafe fn setOrganization(&self, organization: Option<&NSString>);

        #[method_id(@__retain_semantics Other passwordReference)]
        pub unsafe fn passwordReference(&self) -> Option<Retained<NSData>>;

        #[method(setPasswordReference:)]
        pub unsafe fn setPasswordReference(&self, password_reference: Option<&NSData>);

        #[method_id(@__retain_semantics Other identityReference)]
        pub unsafe fn identityReference(&self) -> Option<Retained<NSData>>;

        #[method(setIdentityReference:)]
        pub unsafe fn setIdentityReference(&self, identity_reference: Option<&NSData>);

        #[method_id(@__retain_semantics Other filterDataProviderBundleIdentifier)]
        pub unsafe fn filterDataProviderBundleIdentifier(&self) -> Option<Retained<NSString>>;

        #[method(setFilterDataProviderBundleIdentifier:)]
        pub unsafe fn setFilterDataProviderBundleIdentifier(
            &self,
            filter_data_provider_bundle_identifier: Option<&NSString>,
        );

        #[method_id(@__retain_semantics Other filterPacketProviderBundleIdentifier)]
        pub unsafe fn filterPacketProviderBundleIdentifier(&self) -> Option<Retained<NSString>>;

        #[method(setFilterPacketProviderBundleIdentifier:)]
        pub unsafe fn setFilterPacketProviderBundleIdentifier(
            &self,
            filter_packet_provider_bundle_identifier: Option<&NSString>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEFilterProviderConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

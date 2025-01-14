//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/externalaccessory/eawifiunconfiguredaccessorybrowserstate?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EAWiFiUnconfiguredAccessoryBrowserState(pub NSInteger);
impl EAWiFiUnconfiguredAccessoryBrowserState {
    #[doc(alias = "EAWiFiUnconfiguredAccessoryBrowserStateWiFiUnavailable")]
    pub const WiFiUnavailable: Self = Self(0);
    #[doc(alias = "EAWiFiUnconfiguredAccessoryBrowserStateStopped")]
    pub const Stopped: Self = Self(1);
    #[doc(alias = "EAWiFiUnconfiguredAccessoryBrowserStateSearching")]
    pub const Searching: Self = Self(2);
    #[doc(alias = "EAWiFiUnconfiguredAccessoryBrowserStateConfiguring")]
    pub const Configuring: Self = Self(3);
}

unsafe impl Encode for EAWiFiUnconfiguredAccessoryBrowserState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for EAWiFiUnconfiguredAccessoryBrowserState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/externalaccessory/eawifiunconfiguredaccessoryconfigurationstatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EAWiFiUnconfiguredAccessoryConfigurationStatus(pub NSInteger);
impl EAWiFiUnconfiguredAccessoryConfigurationStatus {
    #[doc(alias = "EAWiFiUnconfiguredAccessoryConfigurationStatusSuccess")]
    pub const Success: Self = Self(0);
    #[doc(alias = "EAWiFiUnconfiguredAccessoryConfigurationStatusUserCancelledConfiguration")]
    pub const UserCancelledConfiguration: Self = Self(1);
    #[doc(alias = "EAWiFiUnconfiguredAccessoryConfigurationStatusFailed")]
    pub const Failed: Self = Self(2);
}

unsafe impl Encode for EAWiFiUnconfiguredAccessoryConfigurationStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for EAWiFiUnconfiguredAccessoryConfigurationStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/externalaccessory/eawifiunconfiguredaccessorybrowser?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct EAWiFiUnconfiguredAccessoryBrowser;
);

unsafe impl NSObjectProtocol for EAWiFiUnconfiguredAccessoryBrowser {}

extern_methods!(
    unsafe impl EAWiFiUnconfiguredAccessoryBrowser {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn EAWiFiUnconfiguredAccessoryBrowserDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn EAWiFiUnconfiguredAccessoryBrowserDelegate>>,
        );

        #[cfg(feature = "EAWiFiUnconfiguredAccessory")]
        #[method_id(@__retain_semantics Other unconfiguredAccessories)]
        pub unsafe fn unconfiguredAccessories(
            &self,
        ) -> Retained<NSSet<EAWiFiUnconfiguredAccessory>>;

        #[method(startSearchingForUnconfiguredAccessoriesMatchingPredicate:)]
        pub unsafe fn startSearchingForUnconfiguredAccessoriesMatchingPredicate(
            &self,
            predicate: Option<&NSPredicate>,
        );

        #[method(stopSearchingForUnconfiguredAccessories)]
        pub unsafe fn stopSearchingForUnconfiguredAccessories(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl EAWiFiUnconfiguredAccessoryBrowser {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/externalaccessory/eawifiunconfiguredaccessorybrowserdelegate?language=objc)
    pub unsafe trait EAWiFiUnconfiguredAccessoryBrowserDelegate: NSObjectProtocol {
        #[method(accessoryBrowser:didUpdateState:)]
        unsafe fn accessoryBrowser_didUpdateState(
            &self,
            browser: &EAWiFiUnconfiguredAccessoryBrowser,
            state: EAWiFiUnconfiguredAccessoryBrowserState,
        );

        #[cfg(feature = "EAWiFiUnconfiguredAccessory")]
        #[method(accessoryBrowser:didFindUnconfiguredAccessories:)]
        unsafe fn accessoryBrowser_didFindUnconfiguredAccessories(
            &self,
            browser: &EAWiFiUnconfiguredAccessoryBrowser,
            accessories: &NSSet<EAWiFiUnconfiguredAccessory>,
        );

        #[cfg(feature = "EAWiFiUnconfiguredAccessory")]
        #[method(accessoryBrowser:didRemoveUnconfiguredAccessories:)]
        unsafe fn accessoryBrowser_didRemoveUnconfiguredAccessories(
            &self,
            browser: &EAWiFiUnconfiguredAccessoryBrowser,
            accessories: &NSSet<EAWiFiUnconfiguredAccessory>,
        );

        #[cfg(feature = "EAWiFiUnconfiguredAccessory")]
        #[method(accessoryBrowser:didFinishConfiguringAccessory:withStatus:)]
        unsafe fn accessoryBrowser_didFinishConfiguringAccessory_withStatus(
            &self,
            browser: &EAWiFiUnconfiguredAccessoryBrowser,
            accessory: &EAWiFiUnconfiguredAccessory,
            status: EAWiFiUnconfiguredAccessoryConfigurationStatus,
        );
    }

    unsafe impl ProtocolType for dyn EAWiFiUnconfiguredAccessoryBrowserDelegate {}
);

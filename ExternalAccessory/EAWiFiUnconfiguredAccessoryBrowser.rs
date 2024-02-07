//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::ExternalAccessory::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum EAWiFiUnconfiguredAccessoryBrowserState {
        #[doc(alias = "EAWiFiUnconfiguredAccessoryBrowserStateWiFiUnavailable")]
        WiFiUnavailable = 0,
        #[doc(alias = "EAWiFiUnconfiguredAccessoryBrowserStateStopped")]
        Stopped = 1,
        #[doc(alias = "EAWiFiUnconfiguredAccessoryBrowserStateSearching")]
        Searching = 2,
        #[doc(alias = "EAWiFiUnconfiguredAccessoryBrowserStateConfiguring")]
        Configuring = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum EAWiFiUnconfiguredAccessoryConfigurationStatus {
        #[doc(alias = "EAWiFiUnconfiguredAccessoryConfigurationStatusSuccess")]
        Success = 0,
        #[doc(alias = "EAWiFiUnconfiguredAccessoryConfigurationStatusUserCancelledConfiguration")]
        UserCancelledConfiguration = 1,
        #[doc(alias = "EAWiFiUnconfiguredAccessoryConfigurationStatusFailed")]
        Failed = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ExternalAccessory_EAWiFiUnconfiguredAccessoryBrowser")]
    pub struct EAWiFiUnconfiguredAccessoryBrowser;

    #[cfg(feature = "ExternalAccessory_EAWiFiUnconfiguredAccessoryBrowser")]
    unsafe impl ClassType for EAWiFiUnconfiguredAccessoryBrowser {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "ExternalAccessory_EAWiFiUnconfiguredAccessoryBrowser")]
unsafe impl NSObjectProtocol for EAWiFiUnconfiguredAccessoryBrowser {}

extern_methods!(
    #[cfg(feature = "ExternalAccessory_EAWiFiUnconfiguredAccessoryBrowser")]
    unsafe impl EAWiFiUnconfiguredAccessoryBrowser {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn EAWiFiUnconfiguredAccessoryBrowserDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn EAWiFiUnconfiguredAccessoryBrowserDelegate>>,
        );

        #[cfg(all(
            feature = "ExternalAccessory_EAWiFiUnconfiguredAccessory",
            feature = "Foundation_NSSet"
        ))]
        #[method_id(@__retain_semantics Other unconfiguredAccessories)]
        pub unsafe fn unconfiguredAccessories(&self) -> Id<NSSet<EAWiFiUnconfiguredAccessory>>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method(startSearchingForUnconfiguredAccessoriesMatchingPredicate:)]
        pub unsafe fn startSearchingForUnconfiguredAccessoriesMatchingPredicate(
            &self,
            predicate: Option<&NSPredicate>,
        );

        #[method(stopSearchingForUnconfiguredAccessories)]
        pub unsafe fn stopSearchingForUnconfiguredAccessories(&self);

        #[cfg(all(
            feature = "ExternalAccessory_EAWiFiUnconfiguredAccessory",
            feature = "ExternalAccessory_UIViewController"
        ))]
        #[method(configureAccessory:withConfigurationUIOnViewController:)]
        pub unsafe fn configureAccessory_withConfigurationUIOnViewController(
            &self,
            accessory: &EAWiFiUnconfiguredAccessory,
            view_controller: &UIViewController,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "ExternalAccessory_EAWiFiUnconfiguredAccessoryBrowser")]
    unsafe impl EAWiFiUnconfiguredAccessoryBrowser {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait EAWiFiUnconfiguredAccessoryBrowserDelegate: NSObjectProtocol {
        #[cfg(feature = "ExternalAccessory_EAWiFiUnconfiguredAccessoryBrowser")]
        #[method(accessoryBrowser:didUpdateState:)]
        unsafe fn accessoryBrowser_didUpdateState(
            &self,
            browser: &EAWiFiUnconfiguredAccessoryBrowser,
            state: EAWiFiUnconfiguredAccessoryBrowserState,
        );

        #[cfg(all(
            feature = "ExternalAccessory_EAWiFiUnconfiguredAccessory",
            feature = "ExternalAccessory_EAWiFiUnconfiguredAccessoryBrowser",
            feature = "Foundation_NSSet"
        ))]
        #[method(accessoryBrowser:didFindUnconfiguredAccessories:)]
        unsafe fn accessoryBrowser_didFindUnconfiguredAccessories(
            &self,
            browser: &EAWiFiUnconfiguredAccessoryBrowser,
            accessories: &NSSet<EAWiFiUnconfiguredAccessory>,
        );

        #[cfg(all(
            feature = "ExternalAccessory_EAWiFiUnconfiguredAccessory",
            feature = "ExternalAccessory_EAWiFiUnconfiguredAccessoryBrowser",
            feature = "Foundation_NSSet"
        ))]
        #[method(accessoryBrowser:didRemoveUnconfiguredAccessories:)]
        unsafe fn accessoryBrowser_didRemoveUnconfiguredAccessories(
            &self,
            browser: &EAWiFiUnconfiguredAccessoryBrowser,
            accessories: &NSSet<EAWiFiUnconfiguredAccessory>,
        );

        #[cfg(all(
            feature = "ExternalAccessory_EAWiFiUnconfiguredAccessory",
            feature = "ExternalAccessory_EAWiFiUnconfiguredAccessoryBrowser"
        ))]
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

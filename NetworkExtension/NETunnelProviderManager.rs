//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/networkextension/netunnelprovidermanager?language=objc)
    #[unsafe(super(NEVPNManager, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NEVPNManager")]
    pub struct NETunnelProviderManager;
);

#[cfg(feature = "NEVPNManager")]
unsafe impl NSObjectProtocol for NETunnelProviderManager {}

extern_methods!(
    #[cfg(feature = "NEVPNManager")]
    unsafe impl NETunnelProviderManager {
        #[cfg(feature = "block2")]
        #[method(loadAllFromPreferencesWithCompletionHandler:)]
        pub unsafe fn loadAllFromPreferencesWithCompletionHandler(
            completion_handler: &block2::Block<
                dyn Fn(*mut NSArray<NETunnelProviderManager>, *mut NSError),
            >,
        );

        #[method_id(@__retain_semantics Other forPerAppVPN)]
        pub unsafe fn forPerAppVPN() -> Retained<Self>;

        #[cfg(feature = "NEAppRule")]
        #[method_id(@__retain_semantics Copy copyAppRules)]
        pub unsafe fn copyAppRules(&self) -> Option<Retained<NSArray<NEAppRule>>>;

        #[cfg(feature = "NETunnelProvider")]
        #[method(routingMethod)]
        pub unsafe fn routingMethod(&self) -> NETunnelProviderRoutingMethod;

        #[method_id(@__retain_semantics Other safariDomains)]
        pub unsafe fn safariDomains(&self) -> Retained<NSArray<NSString>>;

        #[method(setSafariDomains:)]
        pub unsafe fn setSafariDomains(&self, safari_domains: &NSArray<NSString>);

        #[method_id(@__retain_semantics Other mailDomains)]
        pub unsafe fn mailDomains(&self) -> Retained<NSArray<NSString>>;

        #[method(setMailDomains:)]
        pub unsafe fn setMailDomains(&self, mail_domains: &NSArray<NSString>);

        #[method_id(@__retain_semantics Other calendarDomains)]
        pub unsafe fn calendarDomains(&self) -> Retained<NSArray<NSString>>;

        #[method(setCalendarDomains:)]
        pub unsafe fn setCalendarDomains(&self, calendar_domains: &NSArray<NSString>);

        #[method_id(@__retain_semantics Other contactsDomains)]
        pub unsafe fn contactsDomains(&self) -> Retained<NSArray<NSString>>;

        #[method(setContactsDomains:)]
        pub unsafe fn setContactsDomains(&self, contacts_domains: &NSArray<NSString>);

        #[cfg(feature = "NEAppRule")]
        #[method_id(@__retain_semantics Other appRules)]
        pub unsafe fn appRules(&self) -> Retained<NSArray<NEAppRule>>;

        #[cfg(feature = "NEAppRule")]
        #[method(setAppRules:)]
        pub unsafe fn setAppRules(&self, app_rules: &NSArray<NEAppRule>);

        #[method_id(@__retain_semantics Other excludedDomains)]
        pub unsafe fn excludedDomains(&self) -> Retained<NSArray<NSString>>;

        #[method(setExcludedDomains:)]
        pub unsafe fn setExcludedDomains(&self, excluded_domains: &NSArray<NSString>);

        #[method_id(@__retain_semantics Other associatedDomains)]
        pub unsafe fn associatedDomains(&self) -> Retained<NSArray<NSString>>;

        #[method(setAssociatedDomains:)]
        pub unsafe fn setAssociatedDomains(&self, associated_domains: &NSArray<NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NEVPNManager")]
    unsafe impl NETunnelProviderManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

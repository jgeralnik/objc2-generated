//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NERelayManagerError(pub NSInteger);
impl NERelayManagerError {
    #[doc(alias = "NERelayManagerErrorConfigurationInvalid")]
    pub const ConfigurationInvalid: Self = Self(1);
    #[doc(alias = "NERelayManagerErrorConfigurationDisabled")]
    pub const ConfigurationDisabled: Self = Self(2);
    #[doc(alias = "NERelayManagerErrorConfigurationStale")]
    pub const ConfigurationStale: Self = Self(3);
    #[doc(alias = "NERelayManagerErrorConfigurationCannotBeRemoved")]
    pub const ConfigurationCannotBeRemoved: Self = Self(4);
}

unsafe impl Encode for NERelayManagerError {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NERelayManagerError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    pub static NERelayErrorDomain: &'static NSString;
}

extern "C" {
    pub static NERelayConfigurationDidChangeNotification: &'static NSString;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NERelayManager;

    unsafe impl ClassType for NERelayManager {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for NERelayManager {}

extern_methods!(
    unsafe impl NERelayManager {
        #[method_id(@__retain_semantics Other sharedManager)]
        pub unsafe fn sharedManager() -> Retained<NERelayManager>;

        #[cfg(feature = "block2")]
        #[method(loadFromPreferencesWithCompletionHandler:)]
        pub unsafe fn loadFromPreferencesWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(removeFromPreferencesWithCompletionHandler:)]
        pub unsafe fn removeFromPreferencesWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(saveToPreferencesWithCompletionHandler:)]
        pub unsafe fn saveToPreferencesWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[method_id(@__retain_semantics Other localizedDescription)]
        pub unsafe fn localizedDescription(&self) -> Option<Retained<NSString>>;

        #[method(setLocalizedDescription:)]
        pub unsafe fn setLocalizedDescription(&self, localized_description: Option<&NSString>);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[cfg(feature = "NERelay")]
        #[method_id(@__retain_semantics Other relays)]
        pub unsafe fn relays(&self) -> Option<Retained<NSArray<NERelay>>>;

        #[cfg(feature = "NERelay")]
        #[method(setRelays:)]
        pub unsafe fn setRelays(&self, relays: Option<&NSArray<NERelay>>);

        #[method_id(@__retain_semantics Other matchDomains)]
        pub unsafe fn matchDomains(&self) -> Option<Retained<NSArray<NSString>>>;

        #[method(setMatchDomains:)]
        pub unsafe fn setMatchDomains(&self, match_domains: Option<&NSArray<NSString>>);

        #[method_id(@__retain_semantics Other excludedDomains)]
        pub unsafe fn excludedDomains(&self) -> Option<Retained<NSArray<NSString>>>;

        #[method(setExcludedDomains:)]
        pub unsafe fn setExcludedDomains(&self, excluded_domains: Option<&NSArray<NSString>>);

        #[cfg(feature = "NEOnDemandRule")]
        #[method_id(@__retain_semantics Other onDemandRules)]
        pub unsafe fn onDemandRules(&self) -> Option<Retained<NSArray<NEOnDemandRule>>>;

        #[cfg(feature = "NEOnDemandRule")]
        #[method(setOnDemandRules:)]
        pub unsafe fn setOnDemandRules(&self, on_demand_rules: Option<&NSArray<NEOnDemandRule>>);

        #[cfg(feature = "block2")]
        #[method(loadAllManagersFromPreferencesWithCompletionHandler:)]
        pub unsafe fn loadAllManagersFromPreferencesWithCompletionHandler(
            completion_handler: &block2::Block<
                dyn Fn(NonNull<NSArray<NERelayManager>>, *mut NSError),
            >,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NERelayManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

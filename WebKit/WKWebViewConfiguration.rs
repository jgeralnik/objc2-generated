//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkuserinterfacedirectionpolicy?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKUserInterfaceDirectionPolicy(pub NSInteger);
impl WKUserInterfaceDirectionPolicy {
    #[doc(alias = "WKUserInterfaceDirectionPolicyContent")]
    pub const Content: Self = Self(0);
    #[doc(alias = "WKUserInterfaceDirectionPolicySystem")]
    pub const System: Self = Self(1);
}

unsafe impl Encode for WKUserInterfaceDirectionPolicy {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WKUserInterfaceDirectionPolicy {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkaudiovisualmediatypes?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKAudiovisualMediaTypes(pub NSUInteger);
bitflags::bitflags! {
    impl WKAudiovisualMediaTypes: NSUInteger {
        const WKAudiovisualMediaTypeNone = 0;
        const WKAudiovisualMediaTypeAudio = 1<<0;
        const WKAudiovisualMediaTypeVideo = 1<<1;
        const WKAudiovisualMediaTypeAll = NSUIntegerMax as _;
    }
}

unsafe impl Encode for WKAudiovisualMediaTypes {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for WKAudiovisualMediaTypes {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkwebviewconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKWebViewConfiguration;
);

unsafe impl NSCoding for WKWebViewConfiguration {}

unsafe impl NSCopying for WKWebViewConfiguration {}

unsafe impl CopyingHelper for WKWebViewConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for WKWebViewConfiguration {}

unsafe impl NSSecureCoding for WKWebViewConfiguration {}

extern_methods!(
    unsafe impl WKWebViewConfiguration {
        #[cfg(feature = "WKProcessPool")]
        #[method_id(@__retain_semantics Other processPool)]
        pub unsafe fn processPool(&self) -> Retained<WKProcessPool>;

        #[cfg(feature = "WKProcessPool")]
        #[method(setProcessPool:)]
        pub unsafe fn setProcessPool(&self, process_pool: &WKProcessPool);

        #[cfg(feature = "WKPreferences")]
        #[method_id(@__retain_semantics Other preferences)]
        pub unsafe fn preferences(&self) -> Retained<WKPreferences>;

        #[cfg(feature = "WKPreferences")]
        #[method(setPreferences:)]
        pub unsafe fn setPreferences(&self, preferences: &WKPreferences);

        #[cfg(feature = "WKUserContentController")]
        #[method_id(@__retain_semantics Other userContentController)]
        pub unsafe fn userContentController(&self) -> Retained<WKUserContentController>;

        #[cfg(feature = "WKUserContentController")]
        #[method(setUserContentController:)]
        pub unsafe fn setUserContentController(
            &self,
            user_content_controller: &WKUserContentController,
        );

        #[cfg(feature = "WKWebsiteDataStore")]
        #[method_id(@__retain_semantics Other websiteDataStore)]
        pub unsafe fn websiteDataStore(&self) -> Retained<WKWebsiteDataStore>;

        #[cfg(feature = "WKWebsiteDataStore")]
        #[method(setWebsiteDataStore:)]
        pub unsafe fn setWebsiteDataStore(&self, website_data_store: &WKWebsiteDataStore);

        #[method(suppressesIncrementalRendering)]
        pub unsafe fn suppressesIncrementalRendering(&self) -> bool;

        #[method(setSuppressesIncrementalRendering:)]
        pub unsafe fn setSuppressesIncrementalRendering(
            &self,
            suppresses_incremental_rendering: bool,
        );

        #[method_id(@__retain_semantics Other applicationNameForUserAgent)]
        pub unsafe fn applicationNameForUserAgent(&self) -> Option<Retained<NSString>>;

        #[method(setApplicationNameForUserAgent:)]
        pub unsafe fn setApplicationNameForUserAgent(
            &self,
            application_name_for_user_agent: Option<&NSString>,
        );

        #[method(allowsAirPlayForMediaPlayback)]
        pub unsafe fn allowsAirPlayForMediaPlayback(&self) -> bool;

        #[method(setAllowsAirPlayForMediaPlayback:)]
        pub unsafe fn setAllowsAirPlayForMediaPlayback(
            &self,
            allows_air_play_for_media_playback: bool,
        );

        #[method(upgradeKnownHostsToHTTPS)]
        pub unsafe fn upgradeKnownHostsToHTTPS(&self) -> bool;

        #[method(setUpgradeKnownHostsToHTTPS:)]
        pub unsafe fn setUpgradeKnownHostsToHTTPS(&self, upgrade_known_hosts_to_https: bool);

        #[method(mediaTypesRequiringUserActionForPlayback)]
        pub unsafe fn mediaTypesRequiringUserActionForPlayback(&self) -> WKAudiovisualMediaTypes;

        #[method(setMediaTypesRequiringUserActionForPlayback:)]
        pub unsafe fn setMediaTypesRequiringUserActionForPlayback(
            &self,
            media_types_requiring_user_action_for_playback: WKAudiovisualMediaTypes,
        );

        #[cfg(feature = "WKWebpagePreferences")]
        #[method_id(@__retain_semantics Other defaultWebpagePreferences)]
        pub unsafe fn defaultWebpagePreferences(&self) -> Retained<WKWebpagePreferences>;

        #[cfg(feature = "WKWebpagePreferences")]
        #[method(setDefaultWebpagePreferences:)]
        pub unsafe fn setDefaultWebpagePreferences(
            &self,
            default_webpage_preferences: Option<&WKWebpagePreferences>,
        );

        #[method(limitsNavigationsToAppBoundDomains)]
        pub unsafe fn limitsNavigationsToAppBoundDomains(&self) -> bool;

        #[method(setLimitsNavigationsToAppBoundDomains:)]
        pub unsafe fn setLimitsNavigationsToAppBoundDomains(
            &self,
            limits_navigations_to_app_bound_domains: bool,
        );

        #[method(allowsInlinePredictions)]
        pub unsafe fn allowsInlinePredictions(&self) -> bool;

        #[method(setAllowsInlinePredictions:)]
        pub unsafe fn setAllowsInlinePredictions(&self, allows_inline_predictions: bool);

        #[method(userInterfaceDirectionPolicy)]
        pub unsafe fn userInterfaceDirectionPolicy(&self) -> WKUserInterfaceDirectionPolicy;

        #[method(setUserInterfaceDirectionPolicy:)]
        pub unsafe fn setUserInterfaceDirectionPolicy(
            &self,
            user_interface_direction_policy: WKUserInterfaceDirectionPolicy,
        );

        #[cfg(feature = "WKURLSchemeHandler")]
        #[method(setURLSchemeHandler:forURLScheme:)]
        pub unsafe fn setURLSchemeHandler_forURLScheme(
            &self,
            url_scheme_handler: Option<&ProtocolObject<dyn WKURLSchemeHandler>>,
            url_scheme: &NSString,
        );

        #[cfg(feature = "WKURLSchemeHandler")]
        #[method_id(@__retain_semantics Other urlSchemeHandlerForURLScheme:)]
        pub unsafe fn urlSchemeHandlerForURLScheme(
            &self,
            url_scheme: &NSString,
        ) -> Option<Retained<ProtocolObject<dyn WKURLSchemeHandler>>>;

        #[method(supportsAdaptiveImageGlyph)]
        pub unsafe fn supportsAdaptiveImageGlyph(&self) -> bool;

        #[method(setSupportsAdaptiveImageGlyph:)]
        pub unsafe fn setSupportsAdaptiveImageGlyph(&self, supports_adaptive_image_glyph: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKWebViewConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

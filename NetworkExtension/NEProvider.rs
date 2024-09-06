//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NEProviderStopReason(pub NSInteger);
impl NEProviderStopReason {
    #[doc(alias = "NEProviderStopReasonNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "NEProviderStopReasonUserInitiated")]
    pub const UserInitiated: Self = Self(1);
    #[doc(alias = "NEProviderStopReasonProviderFailed")]
    pub const ProviderFailed: Self = Self(2);
    #[doc(alias = "NEProviderStopReasonNoNetworkAvailable")]
    pub const NoNetworkAvailable: Self = Self(3);
    #[doc(alias = "NEProviderStopReasonUnrecoverableNetworkChange")]
    pub const UnrecoverableNetworkChange: Self = Self(4);
    #[doc(alias = "NEProviderStopReasonProviderDisabled")]
    pub const ProviderDisabled: Self = Self(5);
    #[doc(alias = "NEProviderStopReasonAuthenticationCanceled")]
    pub const AuthenticationCanceled: Self = Self(6);
    #[doc(alias = "NEProviderStopReasonConfigurationFailed")]
    pub const ConfigurationFailed: Self = Self(7);
    #[doc(alias = "NEProviderStopReasonIdleTimeout")]
    pub const IdleTimeout: Self = Self(8);
    #[doc(alias = "NEProviderStopReasonConfigurationDisabled")]
    pub const ConfigurationDisabled: Self = Self(9);
    #[doc(alias = "NEProviderStopReasonConfigurationRemoved")]
    pub const ConfigurationRemoved: Self = Self(10);
    #[doc(alias = "NEProviderStopReasonSuperceded")]
    pub const Superceded: Self = Self(11);
    #[doc(alias = "NEProviderStopReasonUserLogout")]
    pub const UserLogout: Self = Self(12);
    #[doc(alias = "NEProviderStopReasonUserSwitch")]
    pub const UserSwitch: Self = Self(13);
    #[doc(alias = "NEProviderStopReasonConnectionFailed")]
    pub const ConnectionFailed: Self = Self(14);
    #[doc(alias = "NEProviderStopReasonSleep")]
    pub const Sleep: Self = Self(15);
    #[doc(alias = "NEProviderStopReasonAppUpdate")]
    pub const AppUpdate: Self = Self(16);
}

unsafe impl Encode for NEProviderStopReason {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NEProviderStopReason {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEProvider;

    unsafe impl ClassType for NEProvider {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for NEProvider {}

extern_methods!(
    unsafe impl NEProvider {
        #[cfg(feature = "block2")]
        #[method(sleepWithCompletionHandler:)]
        pub unsafe fn sleepWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn()>,
        );

        #[method(wake)]
        pub unsafe fn wake(&self);

        #[cfg(all(
            feature = "NWEndpoint",
            feature = "NWTCPConnection",
            feature = "NWTLSParameters"
        ))]
        #[method_id(@__retain_semantics Other createTCPConnectionToEndpoint:enableTLS:TLSParameters:delegate:)]
        pub unsafe fn createTCPConnectionToEndpoint_enableTLS_TLSParameters_delegate(
            &self,
            remote_endpoint: &NWEndpoint,
            enable_tls: bool,
            tls_parameters: Option<&NWTLSParameters>,
            delegate: Option<&AnyObject>,
        ) -> Retained<NWTCPConnection>;

        #[cfg(all(
            feature = "NWEndpoint",
            feature = "NWHostEndpoint",
            feature = "NWUDPSession"
        ))]
        #[method_id(@__retain_semantics Other createUDPSessionToEndpoint:fromEndpoint:)]
        pub unsafe fn createUDPSessionToEndpoint_fromEndpoint(
            &self,
            remote_endpoint: &NWEndpoint,
            local_endpoint: Option<&NWHostEndpoint>,
        ) -> Retained<NWUDPSession>;

        #[cfg(feature = "block2")]
        #[deprecated]
        #[method(displayMessage:completionHandler:)]
        pub unsafe fn displayMessage_completionHandler(
            &self,
            message: &NSString,
            completion_handler: &block2::Block<dyn Fn(Bool)>,
        );

        #[method(startSystemExtensionMode)]
        pub unsafe fn startSystemExtensionMode();

        #[cfg(feature = "NWPath")]
        #[method_id(@__retain_semantics Other defaultPath)]
        pub unsafe fn defaultPath(&self) -> Option<Retained<NWPath>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEProvider {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

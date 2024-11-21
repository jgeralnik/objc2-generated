//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NETunnelProvider, NEProvider, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NEProvider", feature = "NETunnelProvider"))]
    pub struct NEPacketTunnelProvider;
);

#[cfg(all(feature = "NEProvider", feature = "NETunnelProvider"))]
unsafe impl NSObjectProtocol for NEPacketTunnelProvider {}

extern_methods!(
    #[cfg(all(feature = "NEProvider", feature = "NETunnelProvider"))]
    unsafe impl NEPacketTunnelProvider {
        #[cfg(feature = "block2")]
        #[method(startTunnelWithOptions:completionHandler:)]
        pub unsafe fn startTunnelWithOptions_completionHandler(
            &self,
            options: Option<&NSDictionary<NSString, NSObject>>,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(stopTunnelWithReason:completionHandler:)]
        pub unsafe fn stopTunnelWithReason_completionHandler(
            &self,
            reason: NEProviderStopReason,
            completion_handler: &block2::Block<dyn Fn()>,
        );

        #[method(cancelTunnelWithError:)]
        pub unsafe fn cancelTunnelWithError(&self, error: Option<&NSError>);

        #[cfg(feature = "NEPacketTunnelFlow")]
        #[method_id(@__retain_semantics Other packetFlow)]
        pub unsafe fn packetFlow(&self) -> Retained<NEPacketTunnelFlow>;

        #[cfg(all(
            feature = "NWEndpoint",
            feature = "NWTCPConnection",
            feature = "NWTLSParameters"
        ))]
        #[deprecated = "Use the `virtualInterface` property with `nw_parameters_require_interface`"]
        #[method_id(@__retain_semantics Other createTCPConnectionThroughTunnelToEndpoint:enableTLS:TLSParameters:delegate:)]
        pub unsafe fn createTCPConnectionThroughTunnelToEndpoint_enableTLS_TLSParameters_delegate(
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
        #[deprecated = "Use the `virtualInterface` property with `nw_parameters_require_interface`"]
        #[method_id(@__retain_semantics Other createUDPSessionThroughTunnelToEndpoint:fromEndpoint:)]
        pub unsafe fn createUDPSessionThroughTunnelToEndpoint_fromEndpoint(
            &self,
            remote_endpoint: &NWEndpoint,
            local_endpoint: Option<&NWHostEndpoint>,
        ) -> Retained<NWUDPSession>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NEProvider", feature = "NETunnelProvider"))]
    unsafe impl NEPacketTunnelProvider {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

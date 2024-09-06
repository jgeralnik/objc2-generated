//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MCNearbyServiceAdvertiser;

    unsafe impl ClassType for MCNearbyServiceAdvertiser {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for MCNearbyServiceAdvertiser {}

extern_methods!(
    unsafe impl MCNearbyServiceAdvertiser {
        #[cfg(feature = "MCPeerID")]
        #[method_id(@__retain_semantics Init initWithPeer:discoveryInfo:serviceType:)]
        pub unsafe fn initWithPeer_discoveryInfo_serviceType(
            this: Allocated<Self>,
            my_peer_id: &MCPeerID,
            info: Option<&NSDictionary<NSString, NSString>>,
            service_type: &NSString,
        ) -> Retained<Self>;

        #[method(startAdvertisingPeer)]
        pub unsafe fn startAdvertisingPeer(&self);

        #[method(stopAdvertisingPeer)]
        pub unsafe fn stopAdvertisingPeer(&self);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MCNearbyServiceAdvertiserDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn MCNearbyServiceAdvertiserDelegate>>,
        );

        #[cfg(feature = "MCPeerID")]
        #[method_id(@__retain_semantics Other myPeerID)]
        pub unsafe fn myPeerID(&self) -> Retained<MCPeerID>;

        #[method_id(@__retain_semantics Other discoveryInfo)]
        pub unsafe fn discoveryInfo(&self) -> Option<Retained<NSDictionary<NSString, NSString>>>;

        #[method_id(@__retain_semantics Other serviceType)]
        pub unsafe fn serviceType(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MCNearbyServiceAdvertiser {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait MCNearbyServiceAdvertiserDelegate: NSObjectProtocol {
        #[cfg(all(feature = "MCPeerID", feature = "MCSession", feature = "block2"))]
        #[method(advertiser:didReceiveInvitationFromPeer:withContext:invitationHandler:)]
        unsafe fn advertiser_didReceiveInvitationFromPeer_withContext_invitationHandler(
            &self,
            advertiser: &MCNearbyServiceAdvertiser,
            peer_id: &MCPeerID,
            context: Option<&NSData>,
            invitation_handler: &block2::Block<dyn Fn(Bool, *mut MCSession)>,
        );

        #[optional]
        #[method(advertiser:didNotStartAdvertisingPeer:)]
        unsafe fn advertiser_didNotStartAdvertisingPeer(
            &self,
            advertiser: &MCNearbyServiceAdvertiser,
            error: &NSError,
        );
    }

    unsafe impl ProtocolType for dyn MCNearbyServiceAdvertiserDelegate {}
);

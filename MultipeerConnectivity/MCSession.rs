//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MCSessionSendDataMode(pub NSInteger);
impl MCSessionSendDataMode {
    pub const MCSessionSendDataReliable: Self = Self(0);
    pub const MCSessionSendDataUnreliable: Self = Self(1);
}

unsafe impl Encode for MCSessionSendDataMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MCSessionSendDataMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MCSessionState(pub NSInteger);
impl MCSessionState {
    #[doc(alias = "MCSessionStateNotConnected")]
    pub const NotConnected: Self = Self(0);
    #[doc(alias = "MCSessionStateConnecting")]
    pub const Connecting: Self = Self(1);
    #[doc(alias = "MCSessionStateConnected")]
    pub const Connected: Self = Self(2);
}

unsafe impl Encode for MCSessionState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MCSessionState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MCEncryptionPreference(pub NSInteger);
impl MCEncryptionPreference {
    pub const MCEncryptionOptional: Self = Self(0);
    pub const MCEncryptionRequired: Self = Self(1);
    pub const MCEncryptionNone: Self = Self(2);
}

unsafe impl Encode for MCEncryptionPreference {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MCEncryptionPreference {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    pub static kMCSessionMinimumNumberOfPeers: NSUInteger;
}

extern "C" {
    pub static kMCSessionMaximumNumberOfPeers: NSUInteger;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MCSession;

    unsafe impl ClassType for MCSession {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for MCSession {}

extern_methods!(
    unsafe impl MCSession {
        #[cfg(feature = "MCPeerID")]
        #[method_id(@__retain_semantics Init initWithPeer:)]
        pub unsafe fn initWithPeer(this: Allocated<Self>, my_peer_id: &MCPeerID) -> Retained<Self>;

        #[cfg(feature = "MCPeerID")]
        #[method_id(@__retain_semantics Init initWithPeer:securityIdentity:encryptionPreference:)]
        pub unsafe fn initWithPeer_securityIdentity_encryptionPreference(
            this: Allocated<Self>,
            my_peer_id: &MCPeerID,
            identity: Option<&NSArray>,
            encryption_preference: MCEncryptionPreference,
        ) -> Retained<Self>;

        #[cfg(feature = "MCPeerID")]
        #[method(sendData:toPeers:withMode:error:_)]
        pub unsafe fn sendData_toPeers_withMode_error(
            &self,
            data: &NSData,
            peer_i_ds: &NSArray<MCPeerID>,
            mode: MCSessionSendDataMode,
        ) -> Result<(), Retained<NSError>>;

        #[method(disconnect)]
        pub unsafe fn disconnect(&self);

        #[cfg(all(feature = "MCPeerID", feature = "block2"))]
        #[method_id(@__retain_semantics Other sendResourceAtURL:withName:toPeer:withCompletionHandler:)]
        pub unsafe fn sendResourceAtURL_withName_toPeer_withCompletionHandler(
            &self,
            resource_url: &NSURL,
            resource_name: &NSString,
            peer_id: &MCPeerID,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        ) -> Option<Retained<NSProgress>>;

        #[cfg(feature = "MCPeerID")]
        #[method_id(@__retain_semantics Other startStreamWithName:toPeer:error:_)]
        pub unsafe fn startStreamWithName_toPeer_error(
            &self,
            stream_name: &NSString,
            peer_id: &MCPeerID,
        ) -> Result<Retained<NSOutputStream>, Retained<NSError>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn MCSessionDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn MCSessionDelegate>>);

        #[cfg(feature = "MCPeerID")]
        #[method_id(@__retain_semantics Other myPeerID)]
        pub unsafe fn myPeerID(&self) -> Retained<MCPeerID>;

        #[method_id(@__retain_semantics Other securityIdentity)]
        pub unsafe fn securityIdentity(&self) -> Option<Retained<NSArray>>;

        #[method(encryptionPreference)]
        pub unsafe fn encryptionPreference(&self) -> MCEncryptionPreference;

        #[cfg(feature = "MCPeerID")]
        #[method_id(@__retain_semantics Other connectedPeers)]
        pub unsafe fn connectedPeers(&self) -> Retained<NSArray<MCPeerID>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MCSession {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait MCSessionDelegate: NSObjectProtocol {
        #[cfg(feature = "MCPeerID")]
        #[method(session:peer:didChangeState:)]
        unsafe fn session_peer_didChangeState(
            &self,
            session: &MCSession,
            peer_id: &MCPeerID,
            state: MCSessionState,
        );

        #[cfg(feature = "MCPeerID")]
        #[method(session:didReceiveData:fromPeer:)]
        unsafe fn session_didReceiveData_fromPeer(
            &self,
            session: &MCSession,
            data: &NSData,
            peer_id: &MCPeerID,
        );

        #[cfg(feature = "MCPeerID")]
        #[method(session:didReceiveStream:withName:fromPeer:)]
        unsafe fn session_didReceiveStream_withName_fromPeer(
            &self,
            session: &MCSession,
            stream: &NSInputStream,
            stream_name: &NSString,
            peer_id: &MCPeerID,
        );

        #[cfg(feature = "MCPeerID")]
        #[method(session:didStartReceivingResourceWithName:fromPeer:withProgress:)]
        unsafe fn session_didStartReceivingResourceWithName_fromPeer_withProgress(
            &self,
            session: &MCSession,
            resource_name: &NSString,
            peer_id: &MCPeerID,
            progress: &NSProgress,
        );

        #[cfg(feature = "MCPeerID")]
        #[method(session:didFinishReceivingResourceWithName:fromPeer:atURL:withError:)]
        unsafe fn session_didFinishReceivingResourceWithName_fromPeer_atURL_withError(
            &self,
            session: &MCSession,
            resource_name: &NSString,
            peer_id: &MCPeerID,
            local_url: Option<&NSURL>,
            error: Option<&NSError>,
        );

        #[cfg(all(feature = "MCPeerID", feature = "block2"))]
        #[optional]
        #[method(session:didReceiveCertificate:fromPeer:certificateHandler:)]
        unsafe fn session_didReceiveCertificate_fromPeer_certificateHandler(
            &self,
            session: &MCSession,
            certificate: Option<&NSArray>,
            peer_id: &MCPeerID,
            certificate_handler: &block2::Block<dyn Fn(Bool)>,
        );
    }

    unsafe impl ProtocolType for dyn MCSessionDelegate {}
);

extern_methods!(
    /// MCSessionCustomDiscovery
    unsafe impl MCSession {
        #[cfg(all(feature = "MCPeerID", feature = "block2"))]
        #[method(nearbyConnectionDataForPeer:withCompletionHandler:)]
        pub unsafe fn nearbyConnectionDataForPeer_withCompletionHandler(
            &self,
            peer_id: &MCPeerID,
            completion_handler: &block2::Block<dyn Fn(*mut NSData, *mut NSError)>,
        );

        #[cfg(feature = "MCPeerID")]
        #[method(connectPeer:withNearbyConnectionData:)]
        pub unsafe fn connectPeer_withNearbyConnectionData(
            &self,
            peer_id: &MCPeerID,
            data: &NSData,
        );

        #[cfg(feature = "MCPeerID")]
        #[method(cancelConnectPeer:)]
        pub unsafe fn cancelConnectPeer(&self, peer_id: &MCPeerID);
    }
);

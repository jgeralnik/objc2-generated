//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkmatchsenddatamode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GKMatchSendDataMode(pub NSInteger);
impl GKMatchSendDataMode {
    pub const GKMatchSendDataReliable: Self = Self(0);
    pub const GKMatchSendDataUnreliable: Self = Self(1);
}

unsafe impl Encode for GKMatchSendDataMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for GKMatchSendDataMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkplayerconnectionstate?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GKPlayerConnectionState(pub NSInteger);
impl GKPlayerConnectionState {
    pub const GKPlayerStateUnknown: Self = Self(0);
    pub const GKPlayerStateConnected: Self = Self(1);
    pub const GKPlayerStateDisconnected: Self = Self(2);
}

unsafe impl Encode for GKPlayerConnectionState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for GKPlayerConnectionState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkmatch?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKMatch;
);

unsafe impl NSObjectProtocol for GKMatch {}

extern_methods!(
    unsafe impl GKMatch {
        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer"))]
        #[method_id(@__retain_semantics Other players)]
        pub unsafe fn players(&self) -> Retained<NSArray<GKPlayer>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn GKMatchDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn GKMatchDelegate>>);

        #[method(expectedPlayerCount)]
        pub unsafe fn expectedPlayerCount(&self) -> NSUInteger;

        #[cfg(feature = "GKDefines")]
        #[method_id(@__retain_semantics Other properties)]
        pub unsafe fn properties(&self) -> Option<Retained<GKMatchProperties>>;

        #[cfg(all(feature = "GKBasePlayer", feature = "GKDefines", feature = "GKPlayer"))]
        #[method_id(@__retain_semantics Other playerProperties)]
        pub unsafe fn playerProperties(
            &self,
        ) -> Option<Retained<NSDictionary<GKPlayer, GKMatchProperties>>>;

        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer"))]
        #[method(sendData:toPlayers:dataMode:error:_)]
        pub unsafe fn sendData_toPlayers_dataMode_error(
            &self,
            data: &NSData,
            players: &NSArray<GKPlayer>,
            mode: GKMatchSendDataMode,
        ) -> Result<(), Retained<NSError>>;

        #[method(sendDataToAllPlayers:withDataMode:error:_)]
        pub unsafe fn sendDataToAllPlayers_withDataMode_error(
            &self,
            data: &NSData,
            mode: GKMatchSendDataMode,
        ) -> Result<(), Retained<NSError>>;

        #[method(disconnect)]
        pub unsafe fn disconnect(&self);

        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer", feature = "block2"))]
        #[method(chooseBestHostingPlayerWithCompletionHandler:)]
        pub unsafe fn chooseBestHostingPlayerWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut GKPlayer)>,
        );

        #[cfg(feature = "block2")]
        #[method(rematchWithCompletionHandler:)]
        pub unsafe fn rematchWithCompletionHandler(
            &self,
            completion_handler: Option<&block2::Block<dyn Fn(*mut GKMatch, *mut NSError)>>,
        );

        #[cfg(feature = "GKVoiceChat")]
        #[deprecated = "No longer supported"]
        #[method_id(@__retain_semantics Other voiceChatWithName:)]
        pub unsafe fn voiceChatWithName(&self, name: &NSString) -> Option<Retained<GKVoiceChat>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GKMatch {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkmatchdelegate?language=objc)
    pub unsafe trait GKMatchDelegate: NSObjectProtocol {
        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer"))]
        #[optional]
        #[method(match:didReceiveData:fromRemotePlayer:)]
        unsafe fn match_didReceiveData_fromRemotePlayer(
            &self,
            r#match: &GKMatch,
            data: &NSData,
            player: &GKPlayer,
        );

        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer"))]
        #[optional]
        #[method(match:didReceiveData:forRecipient:fromRemotePlayer:)]
        unsafe fn match_didReceiveData_forRecipient_fromRemotePlayer(
            &self,
            r#match: &GKMatch,
            data: &NSData,
            recipient: &GKPlayer,
            player: &GKPlayer,
        );

        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer"))]
        #[optional]
        #[method(match:player:didChangeConnectionState:)]
        unsafe fn match_player_didChangeConnectionState(
            &self,
            r#match: &GKMatch,
            player: &GKPlayer,
            state: GKPlayerConnectionState,
        );

        #[optional]
        #[method(match:didFailWithError:)]
        unsafe fn match_didFailWithError(&self, r#match: &GKMatch, error: Option<&NSError>);

        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer"))]
        #[optional]
        #[method(match:shouldReinviteDisconnectedPlayer:)]
        unsafe fn match_shouldReinviteDisconnectedPlayer(
            &self,
            r#match: &GKMatch,
            player: &GKPlayer,
        ) -> bool;

        #[deprecated]
        #[optional]
        #[method(match:didReceiveData:fromPlayer:)]
        unsafe fn match_didReceiveData_fromPlayer(
            &self,
            r#match: &GKMatch,
            data: &NSData,
            player_id: &NSString,
        );

        #[deprecated]
        #[optional]
        #[method(match:player:didChangeState:)]
        unsafe fn match_player_didChangeState(
            &self,
            r#match: &GKMatch,
            player_id: &NSString,
            state: GKPlayerConnectionState,
        );

        #[deprecated]
        #[optional]
        #[method(match:shouldReinvitePlayer:)]
        unsafe fn match_shouldReinvitePlayer(
            &self,
            r#match: &GKMatch,
            player_id: &NSString,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn GKMatchDelegate {}
);

extern_methods!(
    /// Obsoleted
    unsafe impl GKMatch {
        #[cfg(feature = "block2")]
        #[deprecated]
        #[method(chooseBestHostPlayerWithCompletionHandler:)]
        pub unsafe fn chooseBestHostPlayerWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut NSString)>,
        );

        #[deprecated]
        #[method(sendData:toPlayers:withDataMode:error:_)]
        pub unsafe fn sendData_toPlayers_withDataMode_error(
            &self,
            data: &NSData,
            player_i_ds: &NSArray<NSString>,
            mode: GKMatchSendDataMode,
        ) -> Result<(), Retained<NSError>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other playerIDs)]
        pub unsafe fn playerIDs(&self) -> Option<Retained<NSArray<NSString>>>;
    }
);

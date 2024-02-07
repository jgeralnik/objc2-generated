//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKLocalPlayer")]
    pub struct GKLocalPlayer;

    #[cfg(feature = "GameKit_GKLocalPlayer")]
    unsafe impl ClassType for GKLocalPlayer {
        #[inherits(GKBasePlayer, NSObject)]
        type Super = GKPlayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameKit_GKLocalPlayer")]
unsafe impl NSObjectProtocol for GKLocalPlayer {}

extern_methods!(
    #[cfg(feature = "GameKit_GKLocalPlayer")]
    unsafe impl GKLocalPlayer {
        #[method_id(@__retain_semantics Other local)]
        pub unsafe fn local() -> Id<GKLocalPlayer>;

        #[method_id(@__retain_semantics Other localPlayer)]
        pub unsafe fn localPlayer() -> Id<GKLocalPlayer>;

        #[method(isAuthenticated)]
        pub unsafe fn isAuthenticated(&self) -> bool;

        #[method(isUnderage)]
        pub unsafe fn isUnderage(&self) -> bool;

        #[method(isMultiplayerGamingRestricted)]
        pub unsafe fn isMultiplayerGamingRestricted(&self) -> bool;

        #[method(isPersonalizedCommunicationRestricted)]
        pub unsafe fn isPersonalizedCommunicationRestricted(&self) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[method(loadRecentPlayersWithCompletionHandler:)]
        pub unsafe fn loadRecentPlayersWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<dyn Fn(*mut NSArray<GKPlayer>, *mut NSError)>>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[method(loadChallengableFriendsWithCompletionHandler:)]
        pub unsafe fn loadChallengableFriendsWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<dyn Fn(*mut NSArray<GKPlayer>, *mut NSError)>>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(setDefaultLeaderboardIdentifier:completionHandler:)]
        pub unsafe fn setDefaultLeaderboardIdentifier_completionHandler(
            &self,
            leaderboard_identifier: &NSString,
            completion_handler: Option<&Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(loadDefaultLeaderboardIdentifierWithCompletionHandler:)]
        pub unsafe fn loadDefaultLeaderboardIdentifierWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<dyn Fn(*mut NSString, *mut NSError)>>,
        );

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL"
        ))]
        #[method(fetchItemsForIdentityVerificationSignature:)]
        pub unsafe fn fetchItemsForIdentityVerificationSignature(
            &self,
            completion_handler: Option<
                &Block<dyn Fn(*mut NSURL, *mut NSData, *mut NSData, u64, *mut NSError)>,
            >,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `GKPlayer`
    #[cfg(feature = "GameKit_GKLocalPlayer")]
    unsafe impl GKLocalPlayer {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other anonymousGuestPlayerWithIdentifier:)]
        pub unsafe fn anonymousGuestPlayerWithIdentifier(guest_identifier: &NSString) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GameKit_GKLocalPlayer")]
    unsafe impl GKLocalPlayer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait GKLocalPlayerListener:
        GKChallengeListener + GKInviteEventListener + GKSavedGameListener + GKTurnBasedEventListener
    {
    }

    unsafe impl ProtocolType for dyn GKLocalPlayerListener {}
);

extern_methods!(
    /// GKLocalPlayerEvents
    #[cfg(feature = "GameKit_GKLocalPlayer")]
    unsafe impl GKLocalPlayer {
        #[method(registerListener:)]
        pub unsafe fn registerListener(&self, listener: &ProtocolObject<dyn GKLocalPlayerListener>);

        #[method(unregisterListener:)]
        pub unsafe fn unregisterListener(
            &self,
            listener: &ProtocolObject<dyn GKLocalPlayerListener>,
        );

        #[method(unregisterAllListeners)]
        pub unsafe fn unregisterAllListeners(&self);
    }
);

extern_static!(GKPlayerAuthenticationDidChangeNotificationName: &'static NSNotificationName);

extern_methods!(
    /// Deprecated
    #[cfg(feature = "GameKit_GKLocalPlayer")]
    unsafe impl GKLocalPlayer {
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[deprecated]
        #[method(setDefaultLeaderboardCategoryID:completionHandler:)]
        pub unsafe fn setDefaultLeaderboardCategoryID_completionHandler(
            &self,
            category_id: Option<&NSString>,
            completion_handler: Option<&Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[deprecated]
        #[method(loadDefaultLeaderboardCategoryIDWithCompletionHandler:)]
        pub unsafe fn loadDefaultLeaderboardCategoryIDWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<dyn Fn(*mut NSString, *mut NSError)>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[deprecated]
        #[method(authenticateWithCompletionHandler:)]
        pub unsafe fn authenticateWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[deprecated]
        #[method(loadFriendPlayersWithCompletionHandler:)]
        pub unsafe fn loadFriendPlayersWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<dyn Fn(*mut NSArray<GKPlayer>, *mut NSError)>>,
        );

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL"
        ))]
        #[deprecated]
        #[method(generateIdentityVerificationSignatureWithCompletionHandler:)]
        pub unsafe fn generateIdentityVerificationSignatureWithCompletionHandler(
            &self,
            completion_handler: Option<
                &Block<dyn Fn(*mut NSURL, *mut NSData, *mut NSData, u64, *mut NSError)>,
            >,
        );
    }
);

extern_methods!(
    /// Obsoleted
    #[cfg(feature = "GameKit_GKLocalPlayer")]
    unsafe impl GKLocalPlayer {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[deprecated]
        #[method(loadFriendsWithCompletionHandler:)]
        pub unsafe fn loadFriendsWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<dyn Fn(*mut NSArray<NSString>, *mut NSError)>>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other friends)]
        pub unsafe fn friends(&self) -> Option<Id<NSArray<NSString>>>;
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GKFriendsAuthorizationStatus {
        #[doc(alias = "GKFriendsAuthorizationStatusNotDetermined")]
        NotDetermined = 0,
        #[doc(alias = "GKFriendsAuthorizationStatusRestricted")]
        Restricted = 1,
        #[doc(alias = "GKFriendsAuthorizationStatusDenied")]
        Denied = 2,
        #[doc(alias = "GKFriendsAuthorizationStatusAuthorized")]
        Authorized = 3,
    }
);

extern_methods!(
    /// FriendsList
    #[cfg(feature = "GameKit_GKLocalPlayer")]
    unsafe impl GKLocalPlayer {
        #[cfg(feature = "Foundation_NSError")]
        #[method(loadFriendsAuthorizationStatus:)]
        pub unsafe fn loadFriendsAuthorizationStatus(
            &self,
            completion_handler: &Block<dyn Fn(GKFriendsAuthorizationStatus, *mut NSError)>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[method(loadFriends:)]
        pub unsafe fn loadFriends(
            &self,
            completion_handler: &Block<dyn Fn(*mut NSArray<GKPlayer>, *mut NSError)>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(loadFriendsWithIdentifiers:completionHandler:)]
        pub unsafe fn loadFriendsWithIdentifiers_completionHandler(
            &self,
            identifiers: &NSArray<NSString>,
            completion_handler: &Block<dyn Fn(*mut NSArray<GKPlayer>, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// UI
    #[cfg(feature = "GameKit_GKLocalPlayer")]
    unsafe impl GKLocalPlayer {
        #[cfg(all(feature = "AppKit_NSViewController", feature = "Foundation_NSError"))]
        #[method(authenticateHandler)]
        pub unsafe fn authenticateHandler(
            &self,
            mtm: MainThreadMarker,
        ) -> *mut Block<dyn Fn(*mut NSViewController, *mut NSError)>;

        #[cfg(all(feature = "AppKit_NSViewController", feature = "Foundation_NSError"))]
        #[method(setAuthenticateHandler:)]
        pub unsafe fn setAuthenticateHandler(
            &self,
            authenticate_handler: Option<&Block<dyn Fn(*mut NSViewController, *mut NSError)>>,
        );

        #[method(isPresentingFriendRequestViewController)]
        pub unsafe fn isPresentingFriendRequestViewController(&self) -> bool;

        #[cfg(all(feature = "AppKit_NSWindow", feature = "Foundation_NSError"))]
        #[method(presentFriendRequestCreatorFromWindow:error:_)]
        pub unsafe fn presentFriendRequestCreatorFromWindow_error(
            &self,
            window: Option<&NSWindow>,
        ) -> Result<(), Id<NSError>>;
    }
);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    #[deprecated]
    pub struct GKFriendRequestComposeViewController;

    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl ClassType for GKFriendRequestComposeViewController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
    }
);

#[cfg(all(feature = "GKDialogController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl GKViewController for GKFriendRequestComposeViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSCoding for GKFriendRequestComposeViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSEditor for GKFriendRequestComposeViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for GKFriendRequestComposeViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSSeguePerforming for GKFriendRequestComposeViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSUserInterfaceItemIdentification for GKFriendRequestComposeViewController {}

extern_methods!(
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl GKFriendRequestComposeViewController {}
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl GKFriendRequestComposeViewController {
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl GKFriendRequestComposeViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl GKFriendRequestComposeViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl GKFriendRequestComposeViewController {
        #[deprecated]
        #[method(maxNumberOfRecipients)]
        pub unsafe fn maxNumberOfRecipients(mtm: MainThreadMarker) -> NSUInteger;

        #[deprecated]
        #[method(setMessage:)]
        pub unsafe fn setMessage(&self, message: Option<&NSString>);

        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer"))]
        #[method(addRecipientPlayers:)]
        pub unsafe fn addRecipientPlayers(&self, players: &NSArray<GKPlayer>);

        #[deprecated]
        #[method(addRecipientsWithPlayerIDs:)]
        pub unsafe fn addRecipientsWithPlayerIDs(&self, player_i_ds: &NSArray<NSString>);

        #[deprecated]
        #[method(addRecipientsWithEmailAddresses:)]
        pub unsafe fn addRecipientsWithEmailAddresses(&self, email_addresses: &NSArray<NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other composeViewDelegate)]
        pub unsafe fn composeViewDelegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn GKFriendRequestComposeViewControllerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[deprecated]
        #[method(setComposeViewDelegate:)]
        pub unsafe fn setComposeViewDelegate(
            &self,
            compose_view_delegate: Option<
                &ProtocolObject<dyn GKFriendRequestComposeViewControllerDelegate>,
            >,
        );
    }
);

extern_protocol!(
    #[deprecated]
    pub unsafe trait GKFriendRequestComposeViewControllerDelegate {
        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[deprecated]
        #[method(friendRequestComposeViewControllerDidFinish:)]
        unsafe fn friendRequestComposeViewControllerDidFinish(
            &self,
            view_controller: &GKFriendRequestComposeViewController,
        );
    }

    unsafe impl ProtocolType for dyn GKFriendRequestComposeViewControllerDelegate {}
);

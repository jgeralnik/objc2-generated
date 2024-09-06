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
    pub struct GKTurnBasedMatchmakerViewController;

    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl ClassType for GKTurnBasedMatchmakerViewController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
    }
);

#[cfg(all(feature = "GKDialogController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl GKViewController for GKTurnBasedMatchmakerViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSCoding for GKTurnBasedMatchmakerViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSEditor for GKTurnBasedMatchmakerViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for GKTurnBasedMatchmakerViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSSeguePerforming for GKTurnBasedMatchmakerViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSUserInterfaceItemIdentification for GKTurnBasedMatchmakerViewController {}

extern_methods!(
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl GKTurnBasedMatchmakerViewController {}
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl GKTurnBasedMatchmakerViewController {
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
    unsafe impl GKTurnBasedMatchmakerViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl GKTurnBasedMatchmakerViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl GKTurnBasedMatchmakerViewController {
        #[method_id(@__retain_semantics Other turnBasedMatchmakerDelegate)]
        pub unsafe fn turnBasedMatchmakerDelegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn GKTurnBasedMatchmakerViewControllerDelegate>>>;

        #[method(setTurnBasedMatchmakerDelegate:)]
        pub unsafe fn setTurnBasedMatchmakerDelegate(
            &self,
            turn_based_matchmaker_delegate: Option<
                &ProtocolObject<dyn GKTurnBasedMatchmakerViewControllerDelegate>,
            >,
        );

        #[method(showExistingMatches)]
        pub unsafe fn showExistingMatches(&self) -> bool;

        #[method(setShowExistingMatches:)]
        pub unsafe fn setShowExistingMatches(&self, show_existing_matches: bool);

        #[cfg(feature = "GKMatchmakerViewController")]
        #[method(matchmakingMode)]
        pub unsafe fn matchmakingMode(&self) -> GKMatchmakingMode;

        #[cfg(feature = "GKMatchmakerViewController")]
        #[method(setMatchmakingMode:)]
        pub unsafe fn setMatchmakingMode(&self, matchmaking_mode: GKMatchmakingMode);

        #[cfg(feature = "GKMatchmaker")]
        #[method_id(@__retain_semantics Init initWithMatchRequest:)]
        pub unsafe fn initWithMatchRequest(
            this: Allocated<Self>,
            request: &GKMatchRequest,
        ) -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait GKTurnBasedMatchmakerViewControllerDelegate: NSObjectProtocol {
        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method(turnBasedMatchmakerViewControllerWasCancelled:)]
        unsafe fn turnBasedMatchmakerViewControllerWasCancelled(
            &self,
            view_controller: &GKTurnBasedMatchmakerViewController,
        );

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method(turnBasedMatchmakerViewController:didFailWithError:)]
        unsafe fn turnBasedMatchmakerViewController_didFailWithError(
            &self,
            view_controller: &GKTurnBasedMatchmakerViewController,
            error: &NSError,
        );

        #[cfg(all(feature = "GKTurnBasedMatch", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        #[deprecated]
        #[optional]
        #[method(turnBasedMatchmakerViewController:didFindMatch:)]
        unsafe fn turnBasedMatchmakerViewController_didFindMatch(
            &self,
            view_controller: &GKTurnBasedMatchmakerViewController,
            r#match: &GKTurnBasedMatch,
        );

        #[cfg(all(feature = "GKTurnBasedMatch", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        #[deprecated]
        #[optional]
        #[method(turnBasedMatchmakerViewController:playerQuitForMatch:)]
        unsafe fn turnBasedMatchmakerViewController_playerQuitForMatch(
            &self,
            view_controller: &GKTurnBasedMatchmakerViewController,
            r#match: &GKTurnBasedMatch,
        );
    }

    unsafe impl ProtocolType for dyn GKTurnBasedMatchmakerViewControllerDelegate {}
);

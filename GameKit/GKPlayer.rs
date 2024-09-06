//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static GKPlayerIDNoLongerAvailable: &'static NSString;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GKBasePlayer")]
    pub struct GKPlayer;

    #[cfg(feature = "GKBasePlayer")]
    unsafe impl ClassType for GKPlayer {
        #[inherits(NSObject)]
        type Super = GKBasePlayer;
    }
);

#[cfg(feature = "GKBasePlayer")]
unsafe impl NSObjectProtocol for GKPlayer {}

extern_methods!(
    #[cfg(feature = "GKBasePlayer")]
    unsafe impl GKPlayer {
        #[method(scopedIDsArePersistent)]
        pub unsafe fn scopedIDsArePersistent(&self) -> bool;

        #[method_id(@__retain_semantics Other gamePlayerID)]
        pub unsafe fn gamePlayerID(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other teamPlayerID)]
        pub unsafe fn teamPlayerID(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other displayName)]
        pub unsafe fn displayName(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other alias)]
        pub unsafe fn alias(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other anonymousGuestPlayerWithIdentifier:)]
        pub unsafe fn anonymousGuestPlayerWithIdentifier(
            guest_identifier: &NSString,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other guestIdentifier)]
        pub unsafe fn guestIdentifier(&self) -> Option<Retained<NSString>>;

        #[method(isInvitable)]
        pub unsafe fn isInvitable(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GKBasePlayer")]
    unsafe impl GKPlayer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GKPhotoSize(pub NSInteger);
impl GKPhotoSize {
    #[doc(alias = "GKPhotoSizeSmall")]
    pub const Small: Self = Self(0);
    #[doc(alias = "GKPhotoSizeNormal")]
    pub const Normal: Self = Self(1);
}

unsafe impl Encode for GKPhotoSize {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for GKPhotoSize {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// UI
    #[cfg(feature = "GKBasePlayer")]
    unsafe impl GKPlayer {
        #[cfg(all(feature = "block2", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        #[method(loadPhotoForSize:withCompletionHandler:)]
        pub unsafe fn loadPhotoForSize_withCompletionHandler(
            &self,
            size: GKPhotoSize,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSImage, *mut NSError)>>,
        );
    }
);

extern "C" {
    pub static GKPlayerDidChangeNotificationName: &'static NSNotificationName;
}

extern_methods!(
    /// Deprecated
    #[cfg(feature = "GKBasePlayer")]
    unsafe impl GKPlayer {
        #[deprecated]
        #[method(isFriend)]
        pub unsafe fn isFriend(&self) -> bool;

        #[deprecated = "Use either the gamePlayerID or teamPlayerID property to identify a player."]
        #[method_id(@__retain_semantics Other playerID)]
        pub unsafe fn playerID(&self) -> Retained<NSString>;

        #[cfg(feature = "block2")]
        #[deprecated]
        #[method(loadPlayersForIdentifiers:withCompletionHandler:)]
        pub unsafe fn loadPlayersForIdentifiers_withCompletionHandler(
            identifiers: &NSArray<NSString>,
            completion_handler: Option<
                &block2::Block<dyn Fn(*mut NSArray<GKPlayer>, *mut NSError)>,
            >,
        );
    }
);

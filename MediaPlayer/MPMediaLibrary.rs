//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MPMediaLibraryAuthorizationStatus {
        #[doc(alias = "MPMediaLibraryAuthorizationStatusNotDetermined")]
        NotDetermined = 0,
        #[doc(alias = "MPMediaLibraryAuthorizationStatusDenied")]
        Denied = 1,
        #[doc(alias = "MPMediaLibraryAuthorizationStatusRestricted")]
        Restricted = 2,
        #[doc(alias = "MPMediaLibraryAuthorizationStatusAuthorized")]
        Authorized = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPMediaLibrary")]
    pub struct MPMediaLibrary;

    #[cfg(feature = "MediaPlayer_MPMediaLibrary")]
    unsafe impl ClassType for MPMediaLibrary {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MediaPlayer_MPMediaLibrary")]
unsafe impl NSCoding for MPMediaLibrary {}

#[cfg(feature = "MediaPlayer_MPMediaLibrary")]
unsafe impl NSObjectProtocol for MPMediaLibrary {}

#[cfg(feature = "MediaPlayer_MPMediaLibrary")]
unsafe impl NSSecureCoding for MPMediaLibrary {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPMediaLibrary")]
    unsafe impl MPMediaLibrary {
        #[method_id(@__retain_semantics Other defaultMediaLibrary)]
        pub unsafe fn defaultMediaLibrary() -> Id<MPMediaLibrary>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other lastModifiedDate)]
        pub unsafe fn lastModifiedDate(&self) -> Id<NSDate>;

        #[method(beginGeneratingLibraryChangeNotifications)]
        pub unsafe fn beginGeneratingLibraryChangeNotifications(&self);

        #[method(endGeneratingLibraryChangeNotifications)]
        pub unsafe fn endGeneratingLibraryChangeNotifications(&self);

        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus() -> MPMediaLibraryAuthorizationStatus;

        #[method(requestAuthorization:)]
        pub unsafe fn requestAuthorization(
            completion_handler: &Block<dyn Fn(MPMediaLibraryAuthorizationStatus)>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "MediaPlayer_MPMediaEntity"
        ))]
        #[method(addItemWithProductID:completionHandler:)]
        pub unsafe fn addItemWithProductID_completionHandler(
            &self,
            product_id: &NSString,
            completion_handler: Option<
                &Block<dyn Fn(NonNull<NSArray<MPMediaEntity>>, *mut NSError)>,
            >,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSUUID",
            feature = "MediaPlayer_MPMediaPlaylist",
            feature = "MediaPlayer_MPMediaPlaylistCreationMetadata"
        ))]
        #[method(getPlaylistWithUUID:creationMetadata:completionHandler:)]
        pub unsafe fn getPlaylistWithUUID_creationMetadata_completionHandler(
            &self,
            uuid: &NSUUID,
            creation_metadata: Option<&MPMediaPlaylistCreationMetadata>,
            completion_handler: &Block<dyn Fn(*mut MPMediaPlaylist, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MediaPlayer_MPMediaLibrary")]
    unsafe impl MPMediaLibrary {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_static!(MPMediaLibraryDidChangeNotification: &'static NSString);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPMediaLibraryAuthorizationStatus(pub NSInteger);
impl MPMediaLibraryAuthorizationStatus {
    #[doc(alias = "MPMediaLibraryAuthorizationStatusNotDetermined")]
    pub const NotDetermined: Self = Self(0);
    #[doc(alias = "MPMediaLibraryAuthorizationStatusDenied")]
    pub const Denied: Self = Self(1);
    #[doc(alias = "MPMediaLibraryAuthorizationStatusRestricted")]
    pub const Restricted: Self = Self(2);
    #[doc(alias = "MPMediaLibraryAuthorizationStatusAuthorized")]
    pub const Authorized: Self = Self(3);
}

unsafe impl Encode for MPMediaLibraryAuthorizationStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MPMediaLibraryAuthorizationStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPMediaLibrary;

    unsafe impl ClassType for MPMediaLibrary {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for MPMediaLibrary {}

unsafe impl NSObjectProtocol for MPMediaLibrary {}

unsafe impl NSSecureCoding for MPMediaLibrary {}

extern_methods!(
    unsafe impl MPMediaLibrary {
        #[method_id(@__retain_semantics Other defaultMediaLibrary)]
        pub unsafe fn defaultMediaLibrary() -> Retained<MPMediaLibrary>;

        #[method_id(@__retain_semantics Other lastModifiedDate)]
        pub unsafe fn lastModifiedDate(&self) -> Retained<NSDate>;

        #[method(beginGeneratingLibraryChangeNotifications)]
        pub unsafe fn beginGeneratingLibraryChangeNotifications(&self);

        #[method(endGeneratingLibraryChangeNotifications)]
        pub unsafe fn endGeneratingLibraryChangeNotifications(&self);

        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus() -> MPMediaLibraryAuthorizationStatus;

        #[cfg(feature = "block2")]
        #[method(requestAuthorization:)]
        pub unsafe fn requestAuthorization(
            completion_handler: &block2::Block<dyn Fn(MPMediaLibraryAuthorizationStatus)>,
        );

        #[cfg(all(feature = "MPMediaEntity", feature = "block2"))]
        #[method(addItemWithProductID:completionHandler:)]
        pub unsafe fn addItemWithProductID_completionHandler(
            &self,
            product_id: &NSString,
            completion_handler: Option<
                &block2::Block<dyn Fn(NonNull<NSArray<MPMediaEntity>>, *mut NSError)>,
            >,
        );

        #[cfg(all(
            feature = "MPMediaEntity",
            feature = "MPMediaItemCollection",
            feature = "MPMediaPlaylist",
            feature = "block2"
        ))]
        #[method(getPlaylistWithUUID:creationMetadata:completionHandler:)]
        pub unsafe fn getPlaylistWithUUID_creationMetadata_completionHandler(
            &self,
            uuid: &NSUUID,
            creation_metadata: Option<&MPMediaPlaylistCreationMetadata>,
            completion_handler: &block2::Block<dyn Fn(*mut MPMediaPlaylist, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPMediaLibrary {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    pub static MPMediaLibraryDidChangeNotification: &'static NSString;
}

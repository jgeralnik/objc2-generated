//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpmediaplaylistattribute?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPMediaPlaylistAttribute(pub NSUInteger);
bitflags::bitflags! {
    impl MPMediaPlaylistAttribute: NSUInteger {
        #[doc(alias = "MPMediaPlaylistAttributeNone")]
        const None = 0;
        #[doc(alias = "MPMediaPlaylistAttributeOnTheGo")]
        const OnTheGo = 1<<0;
        #[doc(alias = "MPMediaPlaylistAttributeSmart")]
        const Smart = 1<<1;
        #[doc(alias = "MPMediaPlaylistAttributeGenius")]
        const Genius = 1<<2;
    }
}

unsafe impl Encode for MPMediaPlaylistAttribute {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MPMediaPlaylistAttribute {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpmediaplaylistpropertypersistentid?language=objc)
    pub static MPMediaPlaylistPropertyPersistentID: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpmediaplaylistpropertycloudglobalid?language=objc)
    pub static MPMediaPlaylistPropertyCloudGlobalID: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpmediaplaylistpropertyname?language=objc)
    pub static MPMediaPlaylistPropertyName: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpmediaplaylistpropertyplaylistattributes?language=objc)
    pub static MPMediaPlaylistPropertyPlaylistAttributes: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpmediaplaylistpropertyseeditems?language=objc)
    pub static MPMediaPlaylistPropertySeedItems: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpmediaplaylistpropertydescriptiontext?language=objc)
    pub static MPMediaPlaylistPropertyDescriptionText: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpmediaplaylistpropertyauthordisplayname?language=objc)
    pub static MPMediaPlaylistPropertyAuthorDisplayName: &'static NSString;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpmediaplaylist?language=objc)
    #[unsafe(super(MPMediaItemCollection, MPMediaEntity, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPMediaEntity", feature = "MPMediaItemCollection"))]
    pub struct MPMediaPlaylist;
);

#[cfg(all(feature = "MPMediaEntity", feature = "MPMediaItemCollection"))]
unsafe impl NSCoding for MPMediaPlaylist {}

#[cfg(all(feature = "MPMediaEntity", feature = "MPMediaItemCollection"))]
unsafe impl NSObjectProtocol for MPMediaPlaylist {}

#[cfg(all(feature = "MPMediaEntity", feature = "MPMediaItemCollection"))]
unsafe impl NSSecureCoding for MPMediaPlaylist {}

extern_methods!(
    #[cfg(all(feature = "MPMediaEntity", feature = "MPMediaItemCollection"))]
    unsafe impl MPMediaPlaylist {
        #[method(persistentID)]
        pub unsafe fn persistentID(&self) -> MPMediaEntityPersistentID;

        #[method_id(@__retain_semantics Other cloudGlobalID)]
        pub unsafe fn cloudGlobalID(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Retained<NSString>>;

        #[method(playlistAttributes)]
        pub unsafe fn playlistAttributes(&self) -> MPMediaPlaylistAttribute;

        #[cfg(feature = "MPMediaItem")]
        #[method_id(@__retain_semantics Other seedItems)]
        pub unsafe fn seedItems(&self) -> Option<Retained<NSArray<MPMediaItem>>>;

        #[method_id(@__retain_semantics Other descriptionText)]
        pub unsafe fn descriptionText(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other authorDisplayName)]
        pub unsafe fn authorDisplayName(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "block2")]
        #[method(addItemWithProductID:completionHandler:)]
        pub unsafe fn addItemWithProductID_completionHandler(
            &self,
            product_id: &NSString,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(all(feature = "MPMediaItem", feature = "block2"))]
        #[method(addMediaItems:completionHandler:)]
        pub unsafe fn addMediaItems_completionHandler(
            &self,
            media_items: &NSArray<MPMediaItem>,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `MPMediaItemCollection`
    #[cfg(all(feature = "MPMediaEntity", feature = "MPMediaItemCollection"))]
    unsafe impl MPMediaPlaylist {
        #[cfg(feature = "MPMediaItem")]
        #[method_id(@__retain_semantics Init initWithItems:)]
        pub unsafe fn initWithItems(
            this: Allocated<Self>,
            items: &NSArray<MPMediaItem>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MPMediaEntity", feature = "MPMediaItemCollection"))]
    unsafe impl MPMediaPlaylist {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpmediaplaylistcreationmetadata?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPMediaPlaylistCreationMetadata;
);

unsafe impl NSObjectProtocol for MPMediaPlaylistCreationMetadata {}

extern_methods!(
    unsafe impl MPMediaPlaylistCreationMetadata {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithName:)]
        pub unsafe fn initWithName(this: Allocated<Self>, name: &NSString) -> Retained<Self>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other authorDisplayName)]
        pub unsafe fn authorDisplayName(&self) -> Retained<NSString>;

        #[method(setAuthorDisplayName:)]
        pub unsafe fn setAuthorDisplayName(&self, author_display_name: Option<&NSString>);

        #[method_id(@__retain_semantics Other descriptionText)]
        pub unsafe fn descriptionText(&self) -> Retained<NSString>;

        #[method(setDescriptionText:)]
        pub unsafe fn setDescriptionText(&self, description_text: &NSString);
    }
);

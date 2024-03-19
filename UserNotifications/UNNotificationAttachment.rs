//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::UserNotifications::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNNotificationAttachment;

    unsafe impl ClassType for UNNotificationAttachment {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for UNNotificationAttachment {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for UNNotificationAttachment {}

unsafe impl NSObjectProtocol for UNNotificationAttachment {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for UNNotificationAttachment {}

extern_methods!(
    unsafe impl UNNotificationAttachment {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Id<NSURL>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Id<NSString>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other attachmentWithIdentifier:URL:options:error:_)]
        pub unsafe fn attachmentWithIdentifier_URL_options_error(
            identifier: &NSString,
            url: &NSURL,
            options: Option<&NSDictionary>,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UNNotificationAttachment {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static UNNotificationAttachmentOptionsTypeHintKey: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static UNNotificationAttachmentOptionsThumbnailHiddenKey: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static UNNotificationAttachmentOptionsThumbnailClippingRectKey: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static UNNotificationAttachmentOptionsThumbnailTimeKey: &'static NSString;
}

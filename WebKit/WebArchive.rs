//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/webarchivepboardtype?language=objc)
    pub static WebArchivePboardType: Option<&'static NSString>;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/webarchive?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct WebArchive;
);

unsafe impl NSCoding for WebArchive {}

unsafe impl NSCopying for WebArchive {}

unsafe impl CopyingHelper for WebArchive {
    type Result = Self;
}

unsafe impl NSObjectProtocol for WebArchive {}

extern_methods!(
    unsafe impl WebArchive {
        #[cfg(feature = "WebResource")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithMainResource:subresources:subframeArchives:)]
        pub unsafe fn initWithMainResource_subresources_subframeArchives(
            this: Allocated<Self>,
            main_resource: Option<&WebResource>,
            subresources: Option<&NSArray>,
            subframe_archives: Option<&NSArray>,
        ) -> Option<Retained<Self>>;

        #[deprecated]
        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(
            this: Allocated<Self>,
            data: Option<&NSData>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "WebResource")]
        #[deprecated]
        #[method_id(@__retain_semantics Other mainResource)]
        pub unsafe fn mainResource(&self) -> Option<Retained<WebResource>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other subresources)]
        pub unsafe fn subresources(&self) -> Retained<NSArray>;

        #[deprecated]
        #[method_id(@__retain_semantics Other subframeArchives)]
        pub unsafe fn subframeArchives(&self) -> Retained<NSArray>;

        #[deprecated]
        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data(&self) -> Retained<NSData>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WebArchive {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

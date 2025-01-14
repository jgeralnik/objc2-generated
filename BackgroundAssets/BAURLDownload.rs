//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/backgroundassets/baurldownload?language=objc)
    #[unsafe(super(BADownload, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "BADownload")]
    pub struct BAURLDownload;
);

#[cfg(feature = "BADownload")]
unsafe impl NSCoding for BAURLDownload {}

#[cfg(feature = "BADownload")]
unsafe impl NSCopying for BAURLDownload {}

#[cfg(feature = "BADownload")]
unsafe impl CopyingHelper for BAURLDownload {
    type Result = Self;
}

#[cfg(feature = "BADownload")]
unsafe impl NSObjectProtocol for BAURLDownload {}

#[cfg(feature = "BADownload")]
unsafe impl NSSecureCoding for BAURLDownload {}

extern_methods!(
    #[cfg(feature = "BADownload")]
    unsafe impl BAURLDownload {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithIdentifier:request:fileSize:applicationGroupIdentifier:)]
        pub unsafe fn initWithIdentifier_request_fileSize_applicationGroupIdentifier(
            this: Allocated<Self>,
            identifier: &NSString,
            request: &NSURLRequest,
            file_size: NSUInteger,
            application_group_identifier: &NSString,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithIdentifier:request:essential:fileSize:applicationGroupIdentifier:priority:)]
        pub unsafe fn initWithIdentifier_request_essential_fileSize_applicationGroupIdentifier_priority(
            this: Allocated<Self>,
            identifier: &NSString,
            request: &NSURLRequest,
            essential: bool,
            file_size: NSUInteger,
            application_group_identifier: &NSString,
            priority: BADownloaderPriority,
        ) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init initWithIdentifier:request:applicationGroupIdentifier:)]
        pub unsafe fn initWithIdentifier_request_applicationGroupIdentifier(
            this: Allocated<Self>,
            identifier: &NSString,
            request: &NSURLRequest,
            application_group_identifier: &NSString,
        ) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init initWithIdentifier:request:applicationGroupIdentifier:priority:)]
        pub unsafe fn initWithIdentifier_request_applicationGroupIdentifier_priority(
            this: Allocated<Self>,
            identifier: &NSString,
            request: &NSURLRequest,
            application_group_identifier: &NSString,
            priority: BADownloaderPriority,
        ) -> Retained<Self>;
    }
);

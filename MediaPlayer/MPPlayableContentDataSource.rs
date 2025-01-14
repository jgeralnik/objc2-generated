//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpplayablecontentdatasource?language=objc)
    #[deprecated = "Use CarPlay framework"]
    pub unsafe trait MPPlayableContentDataSource: NSObjectProtocol {
        #[cfg(feature = "block2")]
        #[deprecated = "Use CarPlay framework"]
        #[optional]
        #[method(beginLoadingChildItemsAtIndexPath:completionHandler:)]
        unsafe fn beginLoadingChildItemsAtIndexPath_completionHandler(
            &self,
            index_path: &NSIndexPath,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[deprecated = "Use CarPlay framework"]
        #[optional]
        #[method(childItemsDisplayPlaybackProgressAtIndexPath:)]
        unsafe fn childItemsDisplayPlaybackProgressAtIndexPath(
            &self,
            index_path: &NSIndexPath,
        ) -> bool;

        #[cfg(all(feature = "MPContentItem", feature = "block2"))]
        #[deprecated = "Use CarPlay framework"]
        #[optional]
        #[method(contentItemForIdentifier:completionHandler:)]
        unsafe fn contentItemForIdentifier_completionHandler(
            &self,
            identifier: &NSString,
            completion_handler: &block2::Block<dyn Fn(*mut MPContentItem, *mut NSError)>,
        );

        #[deprecated = "Use CarPlay framework"]
        #[method(numberOfChildItemsAtIndexPath:)]
        unsafe fn numberOfChildItemsAtIndexPath(&self, index_path: &NSIndexPath) -> NSInteger;

        #[cfg(feature = "MPContentItem")]
        #[deprecated = "Use CarPlay framework"]
        #[method_id(@__retain_semantics Other contentItemAtIndexPath:)]
        unsafe fn contentItemAtIndexPath(
            &self,
            index_path: &NSIndexPath,
        ) -> Option<Retained<MPContentItem>>;
    }

    unsafe impl ProtocolType for dyn MPPlayableContentDataSource {}
);

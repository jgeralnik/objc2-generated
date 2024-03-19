//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::FileProvider::*;
use crate::Foundation::*;
use crate::UniformTypeIdentifiers::*;

extern "C" {
    #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
    pub static NSFileProviderErrorDomain: &'static NSErrorDomain;
}

extern "C" {
    #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
    pub static NSFileProviderErrorCollidingItemKey: &'static NSErrorUserInfoKey;
}

extern "C" {
    #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
    pub static NSFileProviderErrorItemKey: &'static NSErrorUserInfoKey;
}

extern "C" {
    #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
    pub static NSFileProviderErrorNonExistentItemIdentifierKey: &'static NSErrorUserInfoKey;
}

ns_error_enum!(
    #[underlying(NSInteger)]
    pub enum NSFileProviderErrorCode {
        NSFileProviderErrorNotAuthenticated = -1000,
        NSFileProviderErrorFilenameCollision = -1001,
        NSFileProviderErrorSyncAnchorExpired = -1002,
        NSFileProviderErrorPageExpired =
            NSFileProviderErrorCode::NSFileProviderErrorSyncAnchorExpired.0,
        NSFileProviderErrorInsufficientQuota = -1003,
        NSFileProviderErrorServerUnreachable = -1004,
        NSFileProviderErrorNoSuchItem = -1005,
        NSFileProviderErrorDeletionRejected = -1006,
        NSFileProviderErrorDirectoryNotEmpty = -1007,
        NSFileProviderErrorProviderNotFound = -2001,
        NSFileProviderErrorProviderTranslocated = -2002,
        NSFileProviderErrorOlderExtensionVersionRunning = -2003,
        NSFileProviderErrorNewerExtensionVersionFound = -2004,
        NSFileProviderErrorCannotSynchronize = -2005,
        NSFileProviderErrorNonEvictableChildren = -2006,
        NSFileProviderErrorUnsyncedEdits = -2007,
        NSFileProviderErrorNonEvictable = -2008,
        NSFileProviderErrorVersionNoLongerAvailable = -2009,
        NSFileProviderErrorExcludedFromSync = -2010,
        NSFileProviderErrorDomainDisabled = -2011,
        NSFileProviderErrorProviderDomainTemporarilyUnavailable = -2012,
        NSFileProviderErrorProviderDomainNotFound = -2013,
        NSFileProviderErrorApplicationExtensionNotFound = -2014,
    }
);

extern_category!(
    /// Category "NSFileProviderError" on [`NSError`].
    #[doc(alias = "NSFileProviderError")]
    pub unsafe trait NSErrorNSFileProviderError {
        #[cfg(feature = "FileProvider_NSFileProviderItem")]
        #[method_id(@__retain_semantics Other fileProviderErrorForCollisionWithItem:)]
        unsafe fn fileProviderErrorForCollisionWithItem(
            existing_item: &NSFileProviderItem,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "FileProvider_NSFileProviderItem",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other fileProviderErrorForNonExistentItemWithIdentifier:)]
        unsafe fn fileProviderErrorForNonExistentItemWithIdentifier(
            item_identifier: &NSFileProviderItemIdentifier,
        ) -> Id<Self>;

        #[cfg(feature = "FileProvider_NSFileProviderItem")]
        #[method_id(@__retain_semantics Other fileProviderErrorForRejectedDeletionOfItem:)]
        unsafe fn fileProviderErrorForRejectedDeletionOfItem(
            updated_version: &NSFileProviderItem,
        ) -> Id<Self>;
    }

    #[cfg(feature = "Foundation_NSError")]
    unsafe impl NSErrorNSFileProviderError for NSError {}
);

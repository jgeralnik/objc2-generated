//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CallKit::*;
use crate::Foundation::*;

extern "C" {
    #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
    pub static CXErrorDomain: &'static NSErrorDomain;
}

extern "C" {
    #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
    pub static CXErrorDomainIncomingCall: &'static NSErrorDomain;
}

extern "C" {
    #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
    pub static CXErrorDomainRequestTransaction: &'static NSErrorDomain;
}

extern "C" {
    #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
    pub static CXErrorDomainCallDirectoryManager: &'static NSErrorDomain;
}

extern "C" {
    #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
    pub static CXErrorDomainNotificationServiceExtension: &'static NSErrorDomain;
}

ns_error_enum!(
    #[underlying(NSInteger)]
    pub enum CXErrorCode {
        #[doc(alias = "CXErrorCodeUnknownError")]
        UnknownError = 0,
        #[doc(alias = "CXErrorCodeUnentitled")]
        Unentitled = 1,
        #[doc(alias = "CXErrorCodeInvalidArgument")]
        InvalidArgument = 2,
        #[doc(alias = "CXErrorCodeMissingVoIPBackgroundMode")]
        MissingVoIPBackgroundMode = 3,
    }
);

ns_error_enum!(
    #[underlying(NSInteger)]
    pub enum CXErrorCodeIncomingCallError {
        #[doc(alias = "CXErrorCodeIncomingCallErrorUnknown")]
        Unknown = 0,
        #[doc(alias = "CXErrorCodeIncomingCallErrorUnentitled")]
        Unentitled = 1,
        #[doc(alias = "CXErrorCodeIncomingCallErrorCallUUIDAlreadyExists")]
        CallUUIDAlreadyExists = 2,
        #[doc(alias = "CXErrorCodeIncomingCallErrorFilteredByDoNotDisturb")]
        FilteredByDoNotDisturb = 3,
        #[doc(alias = "CXErrorCodeIncomingCallErrorFilteredByBlockList")]
        FilteredByBlockList = 4,
        #[doc(alias = "CXErrorCodeIncomingCallErrorFilteredDuringRestrictedSharingMode")]
        FilteredDuringRestrictedSharingMode = 5,
    }
);

ns_error_enum!(
    #[underlying(NSInteger)]
    pub enum CXErrorCodeRequestTransactionError {
        #[doc(alias = "CXErrorCodeRequestTransactionErrorUnknown")]
        Unknown = 0,
        #[doc(alias = "CXErrorCodeRequestTransactionErrorUnentitled")]
        Unentitled = 1,
        #[doc(alias = "CXErrorCodeRequestTransactionErrorUnknownCallProvider")]
        UnknownCallProvider = 2,
        #[doc(alias = "CXErrorCodeRequestTransactionErrorEmptyTransaction")]
        EmptyTransaction = 3,
        #[doc(alias = "CXErrorCodeRequestTransactionErrorUnknownCallUUID")]
        UnknownCallUUID = 4,
        #[doc(alias = "CXErrorCodeRequestTransactionErrorCallUUIDAlreadyExists")]
        CallUUIDAlreadyExists = 5,
        #[doc(alias = "CXErrorCodeRequestTransactionErrorInvalidAction")]
        InvalidAction = 6,
        #[doc(alias = "CXErrorCodeRequestTransactionErrorMaximumCallGroupsReached")]
        MaximumCallGroupsReached = 7,
    }
);

ns_error_enum!(
    #[underlying(NSInteger)]
    pub enum CXErrorCodeCallDirectoryManagerError {
        #[doc(alias = "CXErrorCodeCallDirectoryManagerErrorUnknown")]
        Unknown = 0,
        #[doc(alias = "CXErrorCodeCallDirectoryManagerErrorNoExtensionFound")]
        NoExtensionFound = 1,
        #[doc(alias = "CXErrorCodeCallDirectoryManagerErrorLoadingInterrupted")]
        LoadingInterrupted = 2,
        #[doc(alias = "CXErrorCodeCallDirectoryManagerErrorEntriesOutOfOrder")]
        EntriesOutOfOrder = 3,
        #[doc(alias = "CXErrorCodeCallDirectoryManagerErrorDuplicateEntries")]
        DuplicateEntries = 4,
        #[doc(alias = "CXErrorCodeCallDirectoryManagerErrorMaximumEntriesExceeded")]
        MaximumEntriesExceeded = 5,
        #[doc(alias = "CXErrorCodeCallDirectoryManagerErrorExtensionDisabled")]
        ExtensionDisabled = 6,
        #[doc(alias = "CXErrorCodeCallDirectoryManagerErrorCurrentlyLoading")]
        CurrentlyLoading = 7,
        #[doc(alias = "CXErrorCodeCallDirectoryManagerErrorUnexpectedIncrementalRemoval")]
        UnexpectedIncrementalRemoval = 8,
    }
);

ns_error_enum!(
    #[underlying(NSInteger)]
    pub enum CXErrorCodeNotificationServiceExtensionError {
        #[doc(alias = "CXErrorCodeNotificationServiceExtensionErrorUnknown")]
        Unknown = 0,
        #[doc(alias = "CXErrorCodeNotificationServiceExtensionErrorInvalidClientProcess")]
        InvalidClientProcess = 1,
        #[doc(
            alias = "CXErrorCodeNotificationServiceExtensionErrorMissingNotificationFilteringEntitlement"
        )]
        MissingNotificationFilteringEntitlement = 2,
    }
);

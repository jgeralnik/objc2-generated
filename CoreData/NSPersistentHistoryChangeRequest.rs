//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistenthistorychangerequest?language=objc)
    #[unsafe(super(NSPersistentStoreRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSPersistentStoreRequest")]
    pub struct NSPersistentHistoryChangeRequest;
);

#[cfg(feature = "NSPersistentStoreRequest")]
unsafe impl NSCopying for NSPersistentHistoryChangeRequest {}

#[cfg(feature = "NSPersistentStoreRequest")]
unsafe impl CopyingHelper for NSPersistentHistoryChangeRequest {
    type Result = Self;
}

#[cfg(feature = "NSPersistentStoreRequest")]
unsafe impl NSObjectProtocol for NSPersistentHistoryChangeRequest {}

extern_methods!(
    #[cfg(feature = "NSPersistentStoreRequest")]
    unsafe impl NSPersistentHistoryChangeRequest {
        #[method_id(@__retain_semantics Other fetchHistoryAfterDate:)]
        pub unsafe fn fetchHistoryAfterDate(date: &NSDate) -> Retained<Self>;

        #[cfg(feature = "NSPersistentHistoryToken")]
        #[method_id(@__retain_semantics Other fetchHistoryAfterToken:)]
        pub unsafe fn fetchHistoryAfterToken(
            token: Option<&NSPersistentHistoryToken>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSPersistentHistoryTransaction")]
        #[method_id(@__retain_semantics Other fetchHistoryAfterTransaction:)]
        pub unsafe fn fetchHistoryAfterTransaction(
            transaction: Option<&NSPersistentHistoryTransaction>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSFetchRequest")]
        #[method_id(@__retain_semantics Other fetchHistoryWithFetchRequest:)]
        pub unsafe fn fetchHistoryWithFetchRequest(
            fetch_request: &NSFetchRequest,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other deleteHistoryBeforeDate:)]
        pub unsafe fn deleteHistoryBeforeDate(date: &NSDate) -> Retained<Self>;

        #[cfg(feature = "NSPersistentHistoryToken")]
        #[method_id(@__retain_semantics Other deleteHistoryBeforeToken:)]
        pub unsafe fn deleteHistoryBeforeToken(
            token: Option<&NSPersistentHistoryToken>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSPersistentHistoryTransaction")]
        #[method_id(@__retain_semantics Other deleteHistoryBeforeTransaction:)]
        pub unsafe fn deleteHistoryBeforeTransaction(
            transaction: Option<&NSPersistentHistoryTransaction>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSPersistentStoreResult")]
        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSPersistentHistoryResultType;

        #[cfg(feature = "NSPersistentStoreResult")]
        #[method(setResultType:)]
        pub unsafe fn setResultType(&self, result_type: NSPersistentHistoryResultType);

        #[cfg(feature = "NSPersistentHistoryToken")]
        #[method_id(@__retain_semantics Other token)]
        pub unsafe fn token(&self) -> Option<Retained<NSPersistentHistoryToken>>;

        #[cfg(feature = "NSFetchRequest")]
        #[method_id(@__retain_semantics Other fetchRequest)]
        pub unsafe fn fetchRequest(&self) -> Option<Retained<NSFetchRequest>>;

        #[cfg(feature = "NSFetchRequest")]
        #[method(setFetchRequest:)]
        pub unsafe fn setFetchRequest(&self, fetch_request: Option<&NSFetchRequest>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSPersistentStoreRequest")]
    unsafe impl NSPersistentHistoryChangeRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

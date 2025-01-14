//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckfetchdatabasechangesoperation?language=objc)
    #[unsafe(super(CKDatabaseOperation, CKOperation, NSOperation, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    pub struct CKFetchDatabaseChangesOperation;
);

#[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
unsafe impl NSObjectProtocol for CKFetchDatabaseChangesOperation {}

extern_methods!(
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    unsafe impl CKFetchDatabaseChangesOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "CKServerChangeToken")]
        #[method_id(@__retain_semantics Init initWithPreviousServerChangeToken:)]
        pub unsafe fn initWithPreviousServerChangeToken(
            this: Allocated<Self>,
            previous_server_change_token: Option<&CKServerChangeToken>,
        ) -> Retained<Self>;

        #[cfg(feature = "CKServerChangeToken")]
        #[method_id(@__retain_semantics Other previousServerChangeToken)]
        pub unsafe fn previousServerChangeToken(&self) -> Option<Retained<CKServerChangeToken>>;

        #[cfg(feature = "CKServerChangeToken")]
        #[method(setPreviousServerChangeToken:)]
        pub unsafe fn setPreviousServerChangeToken(
            &self,
            previous_server_change_token: Option<&CKServerChangeToken>,
        );

        #[method(resultsLimit)]
        pub unsafe fn resultsLimit(&self) -> NSUInteger;

        #[method(setResultsLimit:)]
        pub unsafe fn setResultsLimit(&self, results_limit: NSUInteger);

        #[method(fetchAllChanges)]
        pub unsafe fn fetchAllChanges(&self) -> bool;

        #[method(setFetchAllChanges:)]
        pub unsafe fn setFetchAllChanges(&self, fetch_all_changes: bool);

        #[cfg(all(feature = "CKRecordZoneID", feature = "block2"))]
        #[method(recordZoneWithIDChangedBlock)]
        pub unsafe fn recordZoneWithIDChangedBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<CKRecordZoneID>)>;

        #[cfg(all(feature = "CKRecordZoneID", feature = "block2"))]
        #[method(setRecordZoneWithIDChangedBlock:)]
        pub unsafe fn setRecordZoneWithIDChangedBlock(
            &self,
            record_zone_with_id_changed_block: Option<
                &block2::Block<dyn Fn(NonNull<CKRecordZoneID>)>,
            >,
        );

        #[cfg(all(feature = "CKRecordZoneID", feature = "block2"))]
        #[method(recordZoneWithIDWasDeletedBlock)]
        pub unsafe fn recordZoneWithIDWasDeletedBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<CKRecordZoneID>)>;

        #[cfg(all(feature = "CKRecordZoneID", feature = "block2"))]
        #[method(setRecordZoneWithIDWasDeletedBlock:)]
        pub unsafe fn setRecordZoneWithIDWasDeletedBlock(
            &self,
            record_zone_with_id_was_deleted_block: Option<
                &block2::Block<dyn Fn(NonNull<CKRecordZoneID>)>,
            >,
        );

        #[cfg(all(feature = "CKRecordZoneID", feature = "block2"))]
        #[method(recordZoneWithIDWasPurgedBlock)]
        pub unsafe fn recordZoneWithIDWasPurgedBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<CKRecordZoneID>)>;

        #[cfg(all(feature = "CKRecordZoneID", feature = "block2"))]
        #[method(setRecordZoneWithIDWasPurgedBlock:)]
        pub unsafe fn setRecordZoneWithIDWasPurgedBlock(
            &self,
            record_zone_with_id_was_purged_block: Option<
                &block2::Block<dyn Fn(NonNull<CKRecordZoneID>)>,
            >,
        );

        #[cfg(all(feature = "CKRecordZoneID", feature = "block2"))]
        #[method(recordZoneWithIDWasDeletedDueToUserEncryptedDataResetBlock)]
        pub unsafe fn recordZoneWithIDWasDeletedDueToUserEncryptedDataResetBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<CKRecordZoneID>)>;

        #[cfg(all(feature = "CKRecordZoneID", feature = "block2"))]
        #[method(setRecordZoneWithIDWasDeletedDueToUserEncryptedDataResetBlock:)]
        pub unsafe fn setRecordZoneWithIDWasDeletedDueToUserEncryptedDataResetBlock(
            &self,
            record_zone_with_id_was_deleted_due_to_user_encrypted_data_reset_block: Option<
                &block2::Block<dyn Fn(NonNull<CKRecordZoneID>)>,
            >,
        );

        #[cfg(all(feature = "CKServerChangeToken", feature = "block2"))]
        #[method(changeTokenUpdatedBlock)]
        pub unsafe fn changeTokenUpdatedBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<CKServerChangeToken>)>;

        #[cfg(all(feature = "CKServerChangeToken", feature = "block2"))]
        #[method(setChangeTokenUpdatedBlock:)]
        pub unsafe fn setChangeTokenUpdatedBlock(
            &self,
            change_token_updated_block: Option<
                &block2::Block<dyn Fn(NonNull<CKServerChangeToken>)>,
            >,
        );

        #[cfg(all(feature = "CKServerChangeToken", feature = "block2"))]
        #[method(fetchDatabaseChangesCompletionBlock)]
        pub unsafe fn fetchDatabaseChangesCompletionBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(*mut CKServerChangeToken, Bool, *mut NSError)>;

        #[cfg(all(feature = "CKServerChangeToken", feature = "block2"))]
        #[method(setFetchDatabaseChangesCompletionBlock:)]
        pub unsafe fn setFetchDatabaseChangesCompletionBlock(
            &self,
            fetch_database_changes_completion_block: Option<
                &block2::Block<dyn Fn(*mut CKServerChangeToken, Bool, *mut NSError)>,
            >,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    unsafe impl CKFetchDatabaseChangesOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

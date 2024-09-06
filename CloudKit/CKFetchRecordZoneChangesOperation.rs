//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    pub struct CKFetchRecordZoneChangesOperation;

    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    unsafe impl ClassType for CKFetchRecordZoneChangesOperation {
        #[inherits(CKOperation, NSOperation, NSObject)]
        type Super = CKDatabaseOperation;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
unsafe impl NSObjectProtocol for CKFetchRecordZoneChangesOperation {}

extern_methods!(
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    unsafe impl CKFetchRecordZoneChangesOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Init initWithRecordZoneIDs:configurationsByRecordZoneID:)]
        pub unsafe fn initWithRecordZoneIDs_configurationsByRecordZoneID(
            this: Allocated<Self>,
            record_zone_i_ds: &NSArray<CKRecordZoneID>,
            configurations_by_record_zone_id: Option<
                &NSDictionary<CKRecordZoneID, CKFetchRecordZoneChangesConfiguration>,
            >,
        ) -> Retained<Self>;

        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Other recordZoneIDs)]
        pub unsafe fn recordZoneIDs(&self) -> Option<Retained<NSArray<CKRecordZoneID>>>;

        #[cfg(feature = "CKRecordZoneID")]
        #[method(setRecordZoneIDs:)]
        pub unsafe fn setRecordZoneIDs(&self, record_zone_i_ds: Option<&NSArray<CKRecordZoneID>>);

        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Other configurationsByRecordZoneID)]
        pub unsafe fn configurationsByRecordZoneID(
            &self,
        ) -> Option<Retained<NSDictionary<CKRecordZoneID, CKFetchRecordZoneChangesConfiguration>>>;

        #[cfg(feature = "CKRecordZoneID")]
        #[method(setConfigurationsByRecordZoneID:)]
        pub unsafe fn setConfigurationsByRecordZoneID(
            &self,
            configurations_by_record_zone_id: Option<
                &NSDictionary<CKRecordZoneID, CKFetchRecordZoneChangesConfiguration>,
            >,
        );

        #[method(fetchAllChanges)]
        pub unsafe fn fetchAllChanges(&self) -> bool;

        #[method(setFetchAllChanges:)]
        pub unsafe fn setFetchAllChanges(&self, fetch_all_changes: bool);

        #[cfg(all(feature = "CKRecord", feature = "block2"))]
        #[deprecated = "Use recordWasChangedBlock instead, which surfaces per-record errors"]
        #[method(recordChangedBlock)]
        pub unsafe fn recordChangedBlock(&self) -> *mut block2::Block<dyn Fn(NonNull<CKRecord>)>;

        #[cfg(all(feature = "CKRecord", feature = "block2"))]
        #[deprecated = "Use recordWasChangedBlock instead, which surfaces per-record errors"]
        #[method(setRecordChangedBlock:)]
        pub unsafe fn setRecordChangedBlock(
            &self,
            record_changed_block: Option<&block2::Block<dyn Fn(NonNull<CKRecord>)>>,
        );

        #[cfg(all(feature = "CKRecord", feature = "CKRecordID", feature = "block2"))]
        #[method(recordWasChangedBlock)]
        pub unsafe fn recordWasChangedBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<CKRecordID>, *mut CKRecord, *mut NSError)>;

        #[cfg(all(feature = "CKRecord", feature = "CKRecordID", feature = "block2"))]
        #[method(setRecordWasChangedBlock:)]
        pub unsafe fn setRecordWasChangedBlock(
            &self,
            record_was_changed_block: Option<
                &block2::Block<dyn Fn(NonNull<CKRecordID>, *mut CKRecord, *mut NSError)>,
            >,
        );

        #[cfg(all(feature = "CKRecord", feature = "CKRecordID", feature = "block2"))]
        #[method(recordWithIDWasDeletedBlock)]
        pub unsafe fn recordWithIDWasDeletedBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<CKRecordID>, NonNull<CKRecordType>)>;

        #[cfg(all(feature = "CKRecord", feature = "CKRecordID", feature = "block2"))]
        #[method(setRecordWithIDWasDeletedBlock:)]
        pub unsafe fn setRecordWithIDWasDeletedBlock(
            &self,
            record_with_id_was_deleted_block: Option<
                &block2::Block<dyn Fn(NonNull<CKRecordID>, NonNull<CKRecordType>)>,
            >,
        );

        #[cfg(all(
            feature = "CKRecordZoneID",
            feature = "CKServerChangeToken",
            feature = "block2"
        ))]
        #[method(recordZoneChangeTokensUpdatedBlock)]
        pub unsafe fn recordZoneChangeTokensUpdatedBlock(
            &self,
        ) -> *mut block2::Block<
            dyn Fn(NonNull<CKRecordZoneID>, *mut CKServerChangeToken, *mut NSData),
        >;

        #[cfg(all(
            feature = "CKRecordZoneID",
            feature = "CKServerChangeToken",
            feature = "block2"
        ))]
        #[method(setRecordZoneChangeTokensUpdatedBlock:)]
        pub unsafe fn setRecordZoneChangeTokensUpdatedBlock(
            &self,
            record_zone_change_tokens_updated_block: Option<
                &block2::Block<
                    dyn Fn(NonNull<CKRecordZoneID>, *mut CKServerChangeToken, *mut NSData),
                >,
            >,
        );

        #[cfg(all(
            feature = "CKRecordZoneID",
            feature = "CKServerChangeToken",
            feature = "block2"
        ))]
        #[method(recordZoneFetchCompletionBlock)]
        pub unsafe fn recordZoneFetchCompletionBlock(
            &self,
        ) -> *mut block2::Block<
            dyn Fn(
                NonNull<CKRecordZoneID>,
                *mut CKServerChangeToken,
                *mut NSData,
                Bool,
                *mut NSError,
            ),
        >;

        #[cfg(all(
            feature = "CKRecordZoneID",
            feature = "CKServerChangeToken",
            feature = "block2"
        ))]
        #[method(setRecordZoneFetchCompletionBlock:)]
        pub unsafe fn setRecordZoneFetchCompletionBlock(
            &self,
            record_zone_fetch_completion_block: Option<
                &block2::Block<
                    dyn Fn(
                        NonNull<CKRecordZoneID>,
                        *mut CKServerChangeToken,
                        *mut NSData,
                        Bool,
                        *mut NSError,
                    ),
                >,
            >,
        );

        #[cfg(feature = "block2")]
        #[method(fetchRecordZoneChangesCompletionBlock)]
        pub unsafe fn fetchRecordZoneChangesCompletionBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(*mut NSError)>;

        #[cfg(feature = "block2")]
        #[method(setFetchRecordZoneChangesCompletionBlock:)]
        pub unsafe fn setFetchRecordZoneChangesCompletionBlock(
            &self,
            fetch_record_zone_changes_completion_block: Option<
                &block2::Block<dyn Fn(*mut NSError)>,
            >,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    unsafe impl CKFetchRecordZoneChangesOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// Deprecated
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    unsafe impl CKFetchRecordZoneChangesOperation {
        #[cfg(feature = "CKRecordZoneID")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithRecordZoneIDs:optionsByRecordZoneID:)]
        pub unsafe fn initWithRecordZoneIDs_optionsByRecordZoneID(
            this: Allocated<Self>,
            record_zone_i_ds: &NSArray<CKRecordZoneID>,
            options_by_record_zone_id: Option<
                &NSDictionary<CKRecordZoneID, CKFetchRecordZoneChangesOptions>,
            >,
        ) -> Retained<Self>;

        #[cfg(feature = "CKRecordZoneID")]
        #[deprecated]
        #[method_id(@__retain_semantics Other optionsByRecordZoneID)]
        pub unsafe fn optionsByRecordZoneID(
            &self,
        ) -> Option<Retained<NSDictionary<CKRecordZoneID, CKFetchRecordZoneChangesOptions>>>;

        #[cfg(feature = "CKRecordZoneID")]
        #[deprecated]
        #[method(setOptionsByRecordZoneID:)]
        pub unsafe fn setOptionsByRecordZoneID(
            &self,
            options_by_record_zone_id: Option<
                &NSDictionary<CKRecordZoneID, CKFetchRecordZoneChangesOptions>,
            >,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKFetchRecordZoneChangesConfiguration;

    unsafe impl ClassType for CKFetchRecordZoneChangesConfiguration {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CKFetchRecordZoneChangesConfiguration {}

unsafe impl NSCopying for CKFetchRecordZoneChangesConfiguration {}

unsafe impl CopyingHelper for CKFetchRecordZoneChangesConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CKFetchRecordZoneChangesConfiguration {}

unsafe impl NSSecureCoding for CKFetchRecordZoneChangesConfiguration {}

extern_methods!(
    unsafe impl CKFetchRecordZoneChangesConfiguration {
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

        #[cfg(feature = "CKRecord")]
        #[method_id(@__retain_semantics Other desiredKeys)]
        pub unsafe fn desiredKeys(&self) -> Option<Retained<NSArray<CKRecordFieldKey>>>;

        #[cfg(feature = "CKRecord")]
        #[method(setDesiredKeys:)]
        pub unsafe fn setDesiredKeys(&self, desired_keys: Option<&NSArray<CKRecordFieldKey>>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CKFetchRecordZoneChangesConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct CKFetchRecordZoneChangesOptions;

    unsafe impl ClassType for CKFetchRecordZoneChangesOptions {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CKFetchRecordZoneChangesOptions {}

unsafe impl NSCopying for CKFetchRecordZoneChangesOptions {}

unsafe impl CopyingHelper for CKFetchRecordZoneChangesOptions {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CKFetchRecordZoneChangesOptions {}

unsafe impl NSSecureCoding for CKFetchRecordZoneChangesOptions {}

extern_methods!(
    unsafe impl CKFetchRecordZoneChangesOptions {
        #[cfg(feature = "CKServerChangeToken")]
        #[deprecated]
        #[method_id(@__retain_semantics Other previousServerChangeToken)]
        pub unsafe fn previousServerChangeToken(&self) -> Option<Retained<CKServerChangeToken>>;

        #[cfg(feature = "CKServerChangeToken")]
        #[deprecated]
        #[method(setPreviousServerChangeToken:)]
        pub unsafe fn setPreviousServerChangeToken(
            &self,
            previous_server_change_token: Option<&CKServerChangeToken>,
        );

        #[deprecated]
        #[method(resultsLimit)]
        pub unsafe fn resultsLimit(&self) -> NSUInteger;

        #[deprecated]
        #[method(setResultsLimit:)]
        pub unsafe fn setResultsLimit(&self, results_limit: NSUInteger);

        #[cfg(feature = "CKRecord")]
        #[deprecated]
        #[method_id(@__retain_semantics Other desiredKeys)]
        pub unsafe fn desiredKeys(&self) -> Option<Retained<NSArray<CKRecordFieldKey>>>;

        #[cfg(feature = "CKRecord")]
        #[deprecated]
        #[method(setDesiredKeys:)]
        pub unsafe fn setDesiredKeys(&self, desired_keys: Option<&NSArray<CKRecordFieldKey>>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CKFetchRecordZoneChangesOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

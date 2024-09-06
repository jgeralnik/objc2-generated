//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    pub struct CKFetchRecordZonesOperation;

    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    unsafe impl ClassType for CKFetchRecordZonesOperation {
        #[inherits(CKOperation, NSOperation, NSObject)]
        type Super = CKDatabaseOperation;
    }
);

#[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
unsafe impl NSObjectProtocol for CKFetchRecordZonesOperation {}

extern_methods!(
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    unsafe impl CKFetchRecordZonesOperation {
        #[method_id(@__retain_semantics Other fetchAllRecordZonesOperation)]
        pub unsafe fn fetchAllRecordZonesOperation() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Init initWithRecordZoneIDs:)]
        pub unsafe fn initWithRecordZoneIDs(
            this: Allocated<Self>,
            zone_i_ds: &NSArray<CKRecordZoneID>,
        ) -> Retained<Self>;

        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Other recordZoneIDs)]
        pub unsafe fn recordZoneIDs(&self) -> Option<Retained<NSArray<CKRecordZoneID>>>;

        #[cfg(feature = "CKRecordZoneID")]
        #[method(setRecordZoneIDs:)]
        pub unsafe fn setRecordZoneIDs(&self, record_zone_i_ds: Option<&NSArray<CKRecordZoneID>>);

        #[cfg(all(
            feature = "CKRecordZone",
            feature = "CKRecordZoneID",
            feature = "block2"
        ))]
        #[method(perRecordZoneCompletionBlock)]
        pub unsafe fn perRecordZoneCompletionBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<CKRecordZoneID>, *mut CKRecordZone, *mut NSError)>;

        #[cfg(all(
            feature = "CKRecordZone",
            feature = "CKRecordZoneID",
            feature = "block2"
        ))]
        #[method(setPerRecordZoneCompletionBlock:)]
        pub unsafe fn setPerRecordZoneCompletionBlock(
            &self,
            per_record_zone_completion_block: Option<
                &block2::Block<dyn Fn(NonNull<CKRecordZoneID>, *mut CKRecordZone, *mut NSError)>,
            >,
        );

        #[cfg(all(
            feature = "CKRecordZone",
            feature = "CKRecordZoneID",
            feature = "block2"
        ))]
        #[method(fetchRecordZonesCompletionBlock)]
        pub unsafe fn fetchRecordZonesCompletionBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(*mut NSDictionary<CKRecordZoneID, CKRecordZone>, *mut NSError)>;

        #[cfg(all(
            feature = "CKRecordZone",
            feature = "CKRecordZoneID",
            feature = "block2"
        ))]
        #[method(setFetchRecordZonesCompletionBlock:)]
        pub unsafe fn setFetchRecordZonesCompletionBlock(
            &self,
            fetch_record_zones_completion_block: Option<
                &block2::Block<
                    dyn Fn(*mut NSDictionary<CKRecordZoneID, CKRecordZone>, *mut NSError),
                >,
            >,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    unsafe impl CKFetchRecordZonesOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

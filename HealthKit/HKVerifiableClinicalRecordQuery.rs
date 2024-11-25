//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(HKQuery, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HKQuery")]
    pub struct HKVerifiableClinicalRecordQuery;
);

#[cfg(feature = "HKQuery")]
unsafe impl Send for HKVerifiableClinicalRecordQuery {}

#[cfg(feature = "HKQuery")]
unsafe impl Sync for HKVerifiableClinicalRecordQuery {}

#[cfg(feature = "HKQuery")]
unsafe impl NSObjectProtocol for HKVerifiableClinicalRecordQuery {}

extern_methods!(
    #[cfg(feature = "HKQuery")]
    unsafe impl HKVerifiableClinicalRecordQuery {
        #[method_id(@__retain_semantics Other recordTypes)]
        pub unsafe fn recordTypes(&self) -> Retained<NSArray<NSString>>;

        #[cfg(feature = "HKVerifiableClinicalRecord")]
        #[method_id(@__retain_semantics Other sourceTypes)]
        pub unsafe fn sourceTypes(&self)
            -> Retained<NSArray<HKVerifiableClinicalRecordSourceType>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(all(
            feature = "HKObject",
            feature = "HKSample",
            feature = "HKVerifiableClinicalRecord",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Init initWithRecordTypes:predicate:resultsHandler:)]
        pub unsafe fn initWithRecordTypes_predicate_resultsHandler(
            this: Allocated<Self>,
            record_types: &NSArray<NSString>,
            predicate: Option<&NSPredicate>,
            results_handler: &block2::Block<
                dyn Fn(
                    NonNull<HKVerifiableClinicalRecordQuery>,
                    *mut NSArray<HKVerifiableClinicalRecord>,
                    *mut NSError,
                ),
            >,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "HKObject",
            feature = "HKSample",
            feature = "HKVerifiableClinicalRecord",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Init initWithRecordTypes:sourceTypes:predicate:resultsHandler:)]
        pub unsafe fn initWithRecordTypes_sourceTypes_predicate_resultsHandler(
            this: Allocated<Self>,
            record_types: &NSArray<NSString>,
            source_types: &NSArray<HKVerifiableClinicalRecordSourceType>,
            predicate: Option<&NSPredicate>,
            results_handler: &block2::Block<
                dyn Fn(
                    NonNull<HKVerifiableClinicalRecordQuery>,
                    *mut NSArray<HKVerifiableClinicalRecord>,
                    *mut NSError,
                ),
            >,
        ) -> Retained<Self>;
    }
);

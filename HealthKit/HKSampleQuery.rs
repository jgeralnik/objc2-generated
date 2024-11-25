//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

pub static HKObjectQueryNoLimit: NSUInteger = 0;

extern_class!(
    #[unsafe(super(HKQuery, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HKQuery")]
    pub struct HKSampleQuery;
);

#[cfg(feature = "HKQuery")]
unsafe impl Send for HKSampleQuery {}

#[cfg(feature = "HKQuery")]
unsafe impl Sync for HKSampleQuery {}

#[cfg(feature = "HKQuery")]
unsafe impl NSObjectProtocol for HKSampleQuery {}

extern_methods!(
    #[cfg(feature = "HKQuery")]
    unsafe impl HKSampleQuery {
        #[method(limit)]
        pub unsafe fn limit(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other sortDescriptors)]
        pub unsafe fn sortDescriptors(&self) -> Option<Retained<NSArray<NSSortDescriptor>>>;

        #[cfg(all(
            feature = "HKObject",
            feature = "HKObjectType",
            feature = "HKSample",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Init initWithSampleType:predicate:limit:sortDescriptors:resultsHandler:)]
        pub unsafe fn initWithSampleType_predicate_limit_sortDescriptors_resultsHandler(
            this: Allocated<Self>,
            sample_type: &HKSampleType,
            predicate: Option<&NSPredicate>,
            limit: NSUInteger,
            sort_descriptors: Option<&NSArray<NSSortDescriptor>>,
            results_handler: &block2::Block<
                dyn Fn(NonNull<HKSampleQuery>, *mut NSArray<HKSample>, *mut NSError),
            >,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "HKObject",
            feature = "HKQueryDescriptor",
            feature = "HKSample",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Init initWithQueryDescriptors:limit:resultsHandler:)]
        pub unsafe fn initWithQueryDescriptors_limit_resultsHandler(
            this: Allocated<Self>,
            query_descriptors: &NSArray<HKQueryDescriptor>,
            limit: NSInteger,
            results_handler: &block2::Block<
                dyn Fn(NonNull<HKSampleQuery>, *mut NSArray<HKSample>, *mut NSError),
            >,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "HKObject",
            feature = "HKQueryDescriptor",
            feature = "HKSample",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Init initWithQueryDescriptors:limit:sortDescriptors:resultsHandler:)]
        pub unsafe fn initWithQueryDescriptors_limit_sortDescriptors_resultsHandler(
            this: Allocated<Self>,
            query_descriptors: &NSArray<HKQueryDescriptor>,
            limit: NSInteger,
            sort_descriptors: &NSArray<NSSortDescriptor>,
            results_handler: &block2::Block<
                dyn Fn(NonNull<HKSampleQuery>, *mut NSArray<HKSample>, *mut NSError),
            >,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKQuery`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKSampleQuery {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKSampleQuery {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

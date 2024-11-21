//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(HKQuery, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HKQuery")]
    pub struct HKQuantitySeriesSampleQuery;
);

#[cfg(feature = "HKQuery")]
unsafe impl NSObjectProtocol for HKQuantitySeriesSampleQuery {}

extern_methods!(
    #[cfg(feature = "HKQuery")]
    unsafe impl HKQuantitySeriesSampleQuery {
        #[method(includeSample)]
        pub unsafe fn includeSample(&self) -> bool;

        #[method(setIncludeSample:)]
        pub unsafe fn setIncludeSample(&self, include_sample: bool);

        #[method(orderByQuantitySampleStartDate)]
        pub unsafe fn orderByQuantitySampleStartDate(&self) -> bool;

        #[method(setOrderByQuantitySampleStartDate:)]
        pub unsafe fn setOrderByQuantitySampleStartDate(
            &self,
            order_by_quantity_sample_start_date: bool,
        );

        #[cfg(all(
            feature = "HKObject",
            feature = "HKObjectType",
            feature = "HKQuantity",
            feature = "HKQuantitySample",
            feature = "HKSample",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Init initWithQuantityType:predicate:quantityHandler:)]
        pub unsafe fn initWithQuantityType_predicate_quantityHandler(
            this: Allocated<Self>,
            quantity_type: &HKQuantityType,
            predicate: Option<&NSPredicate>,
            quantity_handler: &block2::Block<
                dyn Fn(
                    NonNull<HKQuantitySeriesSampleQuery>,
                    *mut HKQuantity,
                    *mut NSDateInterval,
                    *mut HKQuantitySample,
                    Bool,
                    *mut NSError,
                ),
            >,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "HKObject",
            feature = "HKQuantity",
            feature = "HKQuantitySample",
            feature = "HKSample",
            feature = "block2"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithSample:quantityHandler:)]
        pub unsafe fn initWithSample_quantityHandler(
            this: Allocated<Self>,
            quantity_sample: &HKQuantitySample,
            quantity_handler: &block2::Block<
                dyn Fn(
                    NonNull<HKQuantitySeriesSampleQuery>,
                    *mut HKQuantity,
                    *mut NSDate,
                    Bool,
                    *mut NSError,
                ),
            >,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKQuery`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKQuantitySeriesSampleQuery {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKQuantitySeriesSampleQuery {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

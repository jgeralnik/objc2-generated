//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKStatisticsCollection;

    unsafe impl ClassType for HKStatisticsCollection {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for HKStatisticsCollection {}

extern_methods!(
    unsafe impl HKStatisticsCollection {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "HKStatistics")]
        #[method_id(@__retain_semantics Other statisticsForDate:)]
        pub unsafe fn statisticsForDate(&self, date: &NSDate) -> Option<Retained<HKStatistics>>;

        #[cfg(all(feature = "HKStatistics", feature = "block2"))]
        #[method(enumerateStatisticsFromDate:toDate:withBlock:)]
        pub unsafe fn enumerateStatisticsFromDate_toDate_withBlock(
            &self,
            start_date: &NSDate,
            end_date: &NSDate,
            block: &block2::Block<dyn Fn(NonNull<HKStatistics>, NonNull<Bool>)>,
        );

        #[cfg(feature = "HKStatistics")]
        #[method_id(@__retain_semantics Other statistics)]
        pub unsafe fn statistics(&self) -> Retained<NSArray<HKStatistics>>;

        #[cfg(feature = "HKSource")]
        #[method_id(@__retain_semantics Other sources)]
        pub unsafe fn sources(&self) -> Retained<NSSet<HKSource>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKStatisticsCollection {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HKQuery")]
    pub struct HKStatisticsCollectionQuery;

    #[cfg(feature = "HKQuery")]
    unsafe impl ClassType for HKStatisticsCollectionQuery {
        #[inherits(NSObject)]
        type Super = HKQuery;
    }
);

#[cfg(feature = "HKQuery")]
unsafe impl NSObjectProtocol for HKStatisticsCollectionQuery {}

extern_methods!(
    #[cfg(feature = "HKQuery")]
    unsafe impl HKStatisticsCollectionQuery {
        #[method_id(@__retain_semantics Other anchorDate)]
        pub unsafe fn anchorDate(&self) -> Retained<NSDate>;

        #[cfg(feature = "HKStatistics")]
        #[method(options)]
        pub unsafe fn options(&self) -> HKStatisticsOptions;

        #[method_id(@__retain_semantics Other intervalComponents)]
        pub unsafe fn intervalComponents(&self) -> Retained<NSDateComponents>;

        #[cfg(feature = "block2")]
        #[method(initialResultsHandler)]
        pub unsafe fn initialResultsHandler(
            &self,
        ) -> *mut block2::Block<
            dyn Fn(NonNull<HKStatisticsCollectionQuery>, *mut HKStatisticsCollection, *mut NSError),
        >;

        #[cfg(feature = "block2")]
        #[method(setInitialResultsHandler:)]
        pub unsafe fn setInitialResultsHandler(
            &self,
            initial_results_handler: Option<
                &block2::Block<
                    dyn Fn(
                        NonNull<HKStatisticsCollectionQuery>,
                        *mut HKStatisticsCollection,
                        *mut NSError,
                    ),
                >,
            >,
        );

        #[cfg(all(feature = "HKStatistics", feature = "block2"))]
        #[method(statisticsUpdateHandler)]
        pub unsafe fn statisticsUpdateHandler(
            &self,
        ) -> *mut block2::Block<
            dyn Fn(
                NonNull<HKStatisticsCollectionQuery>,
                *mut HKStatistics,
                *mut HKStatisticsCollection,
                *mut NSError,
            ),
        >;

        #[cfg(all(feature = "HKStatistics", feature = "block2"))]
        #[method(setStatisticsUpdateHandler:)]
        pub unsafe fn setStatisticsUpdateHandler(
            &self,
            statistics_update_handler: Option<
                &block2::Block<
                    dyn Fn(
                        NonNull<HKStatisticsCollectionQuery>,
                        *mut HKStatistics,
                        *mut HKStatisticsCollection,
                        *mut NSError,
                    ),
                >,
            >,
        );

        #[cfg(all(feature = "HKObjectType", feature = "HKStatistics"))]
        #[method_id(@__retain_semantics Init initWithQuantityType:quantitySamplePredicate:options:anchorDate:intervalComponents:)]
        pub unsafe fn initWithQuantityType_quantitySamplePredicate_options_anchorDate_intervalComponents(
            this: Allocated<Self>,
            quantity_type: &HKQuantityType,
            quantity_sample_predicate: Option<&NSPredicate>,
            options: HKStatisticsOptions,
            anchor_date: &NSDate,
            interval_components: &NSDateComponents,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKQuery`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKStatisticsCollectionQuery {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKStatisticsCollectionQuery {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

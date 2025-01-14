//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkactivitysummaryquery?language=objc)
    #[unsafe(super(HKQuery, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HKQuery")]
    pub struct HKActivitySummaryQuery;
);

#[cfg(feature = "HKQuery")]
unsafe impl Send for HKActivitySummaryQuery {}

#[cfg(feature = "HKQuery")]
unsafe impl Sync for HKActivitySummaryQuery {}

#[cfg(feature = "HKQuery")]
unsafe impl NSObjectProtocol for HKActivitySummaryQuery {}

extern_methods!(
    #[cfg(feature = "HKQuery")]
    unsafe impl HKActivitySummaryQuery {
        #[cfg(all(feature = "HKActivitySummary", feature = "block2"))]
        #[method(updateHandler)]
        pub unsafe fn updateHandler(
            &self,
        ) -> *mut block2::Block<
            dyn Fn(NonNull<HKActivitySummaryQuery>, *mut NSArray<HKActivitySummary>, *mut NSError),
        >;

        #[cfg(all(feature = "HKActivitySummary", feature = "block2"))]
        #[method(setUpdateHandler:)]
        pub unsafe fn setUpdateHandler(
            &self,
            update_handler: Option<
                &block2::Block<
                    dyn Fn(
                        NonNull<HKActivitySummaryQuery>,
                        *mut NSArray<HKActivitySummary>,
                        *mut NSError,
                    ),
                >,
            >,
        );

        #[cfg(all(feature = "HKActivitySummary", feature = "block2"))]
        #[method_id(@__retain_semantics Init initWithPredicate:resultsHandler:)]
        pub unsafe fn initWithPredicate_resultsHandler(
            this: Allocated<Self>,
            predicate: Option<&NSPredicate>,
            handler: &block2::Block<
                dyn Fn(
                    NonNull<HKActivitySummaryQuery>,
                    *mut NSArray<HKActivitySummary>,
                    *mut NSError,
                ),
            >,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKQuery`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKActivitySummaryQuery {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKActivitySummaryQuery {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

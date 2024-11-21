//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(HKSeriesBuilder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HKSeriesBuilder")]
    pub struct HKWorkoutRouteBuilder;
);

#[cfg(feature = "HKSeriesBuilder")]
unsafe impl NSObjectProtocol for HKWorkoutRouteBuilder {}

extern_methods!(
    #[cfg(feature = "HKSeriesBuilder")]
    unsafe impl HKWorkoutRouteBuilder {
        #[cfg(all(feature = "HKDevice", feature = "HKHealthStore"))]
        #[method_id(@__retain_semantics Init initWithHealthStore:device:)]
        pub unsafe fn initWithHealthStore_device(
            this: Allocated<Self>,
            health_store: &HKHealthStore,
            device: Option<&HKDevice>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "block2", feature = "objc2-core-location"))]
        #[method(insertRouteData:completion:)]
        pub unsafe fn insertRouteData_completion(
            &self,
            route_data: &NSArray<CLLocation>,
            completion: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(addMetadata:completion:)]
        pub unsafe fn addMetadata_completion(
            &self,
            metadata: &NSDictionary<NSString, AnyObject>,
            completion: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(all(
            feature = "HKObject",
            feature = "HKSample",
            feature = "HKSeriesSample",
            feature = "HKWorkout",
            feature = "HKWorkoutRoute",
            feature = "block2"
        ))]
        #[method(finishRouteWithWorkout:metadata:completion:)]
        pub unsafe fn finishRouteWithWorkout_metadata_completion(
            &self,
            workout: &HKWorkout,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
            completion: &block2::Block<dyn Fn(*mut HKWorkoutRoute, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `HKSeriesBuilder`
    #[cfg(feature = "HKSeriesBuilder")]
    unsafe impl HKWorkoutRouteBuilder {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HKSeriesBuilder")]
    unsafe impl HKWorkoutRouteBuilder {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

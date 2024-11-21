//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKLiveWorkoutDataSource;
);

unsafe impl NSObjectProtocol for HKLiveWorkoutDataSource {}

extern_methods!(
    unsafe impl HKLiveWorkoutDataSource {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "HKObjectType")]
        #[method_id(@__retain_semantics Other typesToCollect)]
        pub unsafe fn typesToCollect(&self) -> Retained<NSSet<HKQuantityType>>;

        #[cfg(all(feature = "HKHealthStore", feature = "HKWorkoutConfiguration"))]
        #[method_id(@__retain_semantics Init initWithHealthStore:workoutConfiguration:)]
        pub unsafe fn initWithHealthStore_workoutConfiguration(
            this: Allocated<Self>,
            health_store: &HKHealthStore,
            configuration: Option<&HKWorkoutConfiguration>,
        ) -> Retained<Self>;

        #[cfg(feature = "HKObjectType")]
        #[method(enableCollectionForType:predicate:)]
        pub unsafe fn enableCollectionForType_predicate(
            &self,
            quantity_type: &HKQuantityType,
            predicate: Option<&NSPredicate>,
        );

        #[cfg(feature = "HKObjectType")]
        #[method(disableCollectionForType:)]
        pub unsafe fn disableCollectionForType(&self, quantity_type: &HKQuantityType);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKLiveWorkoutDataSource {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

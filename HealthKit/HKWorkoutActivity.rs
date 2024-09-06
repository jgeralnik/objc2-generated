//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKWorkoutActivity;

    unsafe impl ClassType for HKWorkoutActivity {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for HKWorkoutActivity {}

unsafe impl NSCopying for HKWorkoutActivity {}

unsafe impl CopyingHelper for HKWorkoutActivity {
    type Result = Self;
}

unsafe impl NSObjectProtocol for HKWorkoutActivity {}

unsafe impl NSSecureCoding for HKWorkoutActivity {}

extern_methods!(
    unsafe impl HKWorkoutActivity {
        #[method_id(@__retain_semantics Other UUID)]
        pub unsafe fn UUID(&self) -> Retained<NSUUID>;

        #[cfg(feature = "HKWorkoutConfiguration")]
        #[method_id(@__retain_semantics Other workoutConfiguration)]
        pub unsafe fn workoutConfiguration(&self) -> Retained<HKWorkoutConfiguration>;

        #[method_id(@__retain_semantics Other startDate)]
        pub unsafe fn startDate(&self) -> Retained<NSDate>;

        #[method_id(@__retain_semantics Other endDate)]
        pub unsafe fn endDate(&self) -> Option<Retained<NSDate>>;

        #[method_id(@__retain_semantics Other metadata)]
        pub unsafe fn metadata(&self) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;

        #[cfg(feature = "HKWorkout")]
        #[method_id(@__retain_semantics Other workoutEvents)]
        pub unsafe fn workoutEvents(&self) -> Retained<NSArray<HKWorkoutEvent>>;

        #[cfg(all(feature = "HKObjectType", feature = "HKStatistics"))]
        #[method_id(@__retain_semantics Other allStatistics)]
        pub unsafe fn allStatistics(&self) -> Retained<NSDictionary<HKQuantityType, HKStatistics>>;

        #[cfg(all(feature = "HKObjectType", feature = "HKStatistics"))]
        #[method_id(@__retain_semantics Other statisticsForType:)]
        pub unsafe fn statisticsForType(
            &self,
            quantity_type: &HKQuantityType,
        ) -> Option<Retained<HKStatistics>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "HKWorkoutConfiguration")]
        #[method_id(@__retain_semantics Init initWithWorkoutConfiguration:startDate:endDate:metadata:)]
        pub unsafe fn initWithWorkoutConfiguration_startDate_endDate_metadata(
            this: Allocated<Self>,
            workout_configuration: &HKWorkoutConfiguration,
            start_date: &NSDate,
            end_date: Option<&NSDate>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;
    }
);

extern "C" {
    pub static HKPredicateKeyPathWorkoutActivityType: &'static NSString;
}

extern "C" {
    pub static HKPredicateKeyPathWorkoutActivityDuration: &'static NSString;
}

extern "C" {
    pub static HKPredicateKeyPathWorkoutActivityStartDate: &'static NSString;
}

extern "C" {
    pub static HKPredicateKeyPathWorkoutActivityEndDate: &'static NSString;
}

extern "C" {
    pub static HKPredicateKeyPathWorkoutActivitySumQuantity: &'static NSString;
}

extern "C" {
    pub static HKPredicateKeyPathWorkoutActivityMinimumQuantity: &'static NSString;
}

extern "C" {
    pub static HKPredicateKeyPathWorkoutActivityMaximumQuantity: &'static NSString;
}

extern "C" {
    pub static HKPredicateKeyPathWorkoutActivityAverageQuantity: &'static NSString;
}

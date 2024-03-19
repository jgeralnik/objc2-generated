//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKWorkoutActivity;

    unsafe impl ClassType for HKWorkoutActivity {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for HKWorkoutActivity {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for HKWorkoutActivity {}

unsafe impl NSObjectProtocol for HKWorkoutActivity {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for HKWorkoutActivity {}

extern_methods!(
    unsafe impl HKWorkoutActivity {
        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Other UUID)]
        pub unsafe fn UUID(&self) -> Id<NSUUID>;

        #[cfg(feature = "HealthKit_HKWorkoutConfiguration")]
        #[method_id(@__retain_semantics Other workoutConfiguration)]
        pub unsafe fn workoutConfiguration(&self) -> Id<HKWorkoutConfiguration>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other startDate)]
        pub unsafe fn startDate(&self) -> Id<NSDate>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other endDate)]
        pub unsafe fn endDate(&self) -> Option<Id<NSDate>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other metadata)]
        pub unsafe fn metadata(&self) -> Option<Id<NSDictionary<NSString, AnyObject>>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;

        #[cfg(all(feature = "Foundation_NSArray", feature = "HealthKit_HKWorkout"))]
        #[method_id(@__retain_semantics Other workoutEvents)]
        pub unsafe fn workoutEvents(&self) -> Id<NSArray<HKWorkoutEvent>>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "HealthKit_HKObjectType",
            feature = "HealthKit_HKStatistics"
        ))]
        #[method_id(@__retain_semantics Other allStatistics)]
        pub unsafe fn allStatistics(&self) -> Id<NSDictionary<HKQuantityType, HKStatistics>>;

        #[cfg(all(feature = "HealthKit_HKObjectType", feature = "HealthKit_HKStatistics"))]
        #[method_id(@__retain_semantics Other statisticsForType:)]
        pub unsafe fn statisticsForType(
            &self,
            quantity_type: &HKQuantityType,
        ) -> Option<Id<HKStatistics>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "HealthKit_HKWorkoutConfiguration"
        ))]
        #[method_id(@__retain_semantics Init initWithWorkoutConfiguration:startDate:endDate:metadata:)]
        pub unsafe fn initWithWorkoutConfiguration_startDate_endDate_metadata(
            this: Allocated<Self>,
            workout_configuration: &HKWorkoutConfiguration,
            start_date: &NSDate,
            end_date: Option<&NSDate>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Id<Self>;
    }
);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static HKPredicateKeyPathWorkoutActivityType: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static HKPredicateKeyPathWorkoutActivityDuration: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static HKPredicateKeyPathWorkoutActivityStartDate: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static HKPredicateKeyPathWorkoutActivityEndDate: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static HKPredicateKeyPathWorkoutActivitySumQuantity: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static HKPredicateKeyPathWorkoutActivityMinimumQuantity: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static HKPredicateKeyPathWorkoutActivityMaximumQuantity: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static HKPredicateKeyPathWorkoutActivityAverageQuantity: &'static NSString;
}

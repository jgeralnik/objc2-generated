//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKActivitySummary;
);

unsafe impl NSCoding for HKActivitySummary {}

unsafe impl NSCopying for HKActivitySummary {}

unsafe impl CopyingHelper for HKActivitySummary {
    type Result = Self;
}

unsafe impl NSObjectProtocol for HKActivitySummary {}

unsafe impl NSSecureCoding for HKActivitySummary {}

extern_methods!(
    unsafe impl HKActivitySummary {
        #[method_id(@__retain_semantics Other dateComponentsForCalendar:)]
        pub unsafe fn dateComponentsForCalendar(
            &self,
            calendar: &NSCalendar,
        ) -> Retained<NSDateComponents>;

        #[cfg(feature = "HKCharacteristicValues")]
        #[method(activityMoveMode)]
        pub unsafe fn activityMoveMode(&self) -> HKActivityMoveMode;

        #[cfg(feature = "HKCharacteristicValues")]
        #[method(setActivityMoveMode:)]
        pub unsafe fn setActivityMoveMode(&self, activity_move_mode: HKActivityMoveMode);

        #[method(isPaused)]
        pub unsafe fn isPaused(&self) -> bool;

        #[method(setPaused:)]
        pub unsafe fn setPaused(&self, paused: bool);

        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Other activeEnergyBurned)]
        pub unsafe fn activeEnergyBurned(&self) -> Retained<HKQuantity>;

        #[cfg(feature = "HKQuantity")]
        #[method(setActiveEnergyBurned:)]
        pub unsafe fn setActiveEnergyBurned(&self, active_energy_burned: &HKQuantity);

        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Other appleMoveTime)]
        pub unsafe fn appleMoveTime(&self) -> Retained<HKQuantity>;

        #[cfg(feature = "HKQuantity")]
        #[method(setAppleMoveTime:)]
        pub unsafe fn setAppleMoveTime(&self, apple_move_time: &HKQuantity);

        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Other appleExerciseTime)]
        pub unsafe fn appleExerciseTime(&self) -> Retained<HKQuantity>;

        #[cfg(feature = "HKQuantity")]
        #[method(setAppleExerciseTime:)]
        pub unsafe fn setAppleExerciseTime(&self, apple_exercise_time: &HKQuantity);

        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Other appleStandHours)]
        pub unsafe fn appleStandHours(&self) -> Retained<HKQuantity>;

        #[cfg(feature = "HKQuantity")]
        #[method(setAppleStandHours:)]
        pub unsafe fn setAppleStandHours(&self, apple_stand_hours: &HKQuantity);

        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Other activeEnergyBurnedGoal)]
        pub unsafe fn activeEnergyBurnedGoal(&self) -> Retained<HKQuantity>;

        #[cfg(feature = "HKQuantity")]
        #[method(setActiveEnergyBurnedGoal:)]
        pub unsafe fn setActiveEnergyBurnedGoal(&self, active_energy_burned_goal: &HKQuantity);

        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Other appleMoveTimeGoal)]
        pub unsafe fn appleMoveTimeGoal(&self) -> Retained<HKQuantity>;

        #[cfg(feature = "HKQuantity")]
        #[method(setAppleMoveTimeGoal:)]
        pub unsafe fn setAppleMoveTimeGoal(&self, apple_move_time_goal: &HKQuantity);

        #[cfg(feature = "HKQuantity")]
        #[deprecated]
        #[method_id(@__retain_semantics Other appleExerciseTimeGoal)]
        pub unsafe fn appleExerciseTimeGoal(&self) -> Retained<HKQuantity>;

        #[cfg(feature = "HKQuantity")]
        #[deprecated]
        #[method(setAppleExerciseTimeGoal:)]
        pub unsafe fn setAppleExerciseTimeGoal(&self, apple_exercise_time_goal: &HKQuantity);

        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Other exerciseTimeGoal)]
        pub unsafe fn exerciseTimeGoal(&self) -> Option<Retained<HKQuantity>>;

        #[cfg(feature = "HKQuantity")]
        #[method(setExerciseTimeGoal:)]
        pub unsafe fn setExerciseTimeGoal(&self, exercise_time_goal: Option<&HKQuantity>);

        #[cfg(feature = "HKQuantity")]
        #[deprecated]
        #[method_id(@__retain_semantics Other appleStandHoursGoal)]
        pub unsafe fn appleStandHoursGoal(&self) -> Retained<HKQuantity>;

        #[cfg(feature = "HKQuantity")]
        #[deprecated]
        #[method(setAppleStandHoursGoal:)]
        pub unsafe fn setAppleStandHoursGoal(&self, apple_stand_hours_goal: &HKQuantity);

        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Other standHoursGoal)]
        pub unsafe fn standHoursGoal(&self) -> Option<Retained<HKQuantity>>;

        #[cfg(feature = "HKQuantity")]
        #[method(setStandHoursGoal:)]
        pub unsafe fn setStandHoursGoal(&self, stand_hours_goal: Option<&HKQuantity>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKActivitySummary {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    pub static HKPredicateKeyPathDateComponents: &'static NSString;
}

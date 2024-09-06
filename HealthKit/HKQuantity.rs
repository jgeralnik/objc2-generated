//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKQuantity;

    unsafe impl ClassType for HKQuantity {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for HKQuantity {}

unsafe impl Sync for HKQuantity {}

unsafe impl NSCoding for HKQuantity {}

unsafe impl NSCopying for HKQuantity {}

unsafe impl CopyingHelper for HKQuantity {
    type Result = Self;
}

unsafe impl NSObjectProtocol for HKQuantity {}

unsafe impl NSSecureCoding for HKQuantity {}

extern_methods!(
    unsafe impl HKQuantity {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "HKUnit")]
        #[method_id(@__retain_semantics Other quantityWithUnit:doubleValue:)]
        pub unsafe fn quantityWithUnit_doubleValue(
            unit: &HKUnit,
            value: c_double,
        ) -> Retained<Self>;

        #[cfg(feature = "HKUnit")]
        #[method(isCompatibleWithUnit:)]
        pub unsafe fn isCompatibleWithUnit(&self, unit: &HKUnit) -> bool;

        #[cfg(feature = "HKUnit")]
        #[method(doubleValueForUnit:)]
        pub unsafe fn doubleValueForUnit(&self, unit: &HKUnit) -> c_double;

        #[method(compare:)]
        pub unsafe fn compare(&self, quantity: &HKQuantity) -> NSComparisonResult;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKQuantity {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "HealthKit_HKObject", feature = "HealthKit_HKSample"))]
    pub struct HKQuantitySample;

    #[cfg(all(feature = "HealthKit_HKObject", feature = "HealthKit_HKSample"))]
    unsafe impl ClassType for HKQuantitySample {
        #[inherits(HKObject, NSObject)]
        type Super = HKSample;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "HealthKit_HKObject",
    feature = "HealthKit_HKSample"
))]
unsafe impl NSCoding for HKQuantitySample {}

#[cfg(all(feature = "HealthKit_HKObject", feature = "HealthKit_HKSample"))]
unsafe impl NSObjectProtocol for HKQuantitySample {}

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "HealthKit_HKObject",
    feature = "HealthKit_HKSample"
))]
unsafe impl NSSecureCoding for HKQuantitySample {}

extern_methods!(
    #[cfg(all(feature = "HealthKit_HKObject", feature = "HealthKit_HKSample"))]
    unsafe impl HKQuantitySample {
        #[cfg(feature = "HealthKit_HKObjectType")]
        #[method_id(@__retain_semantics Other quantityType)]
        pub unsafe fn quantityType(&self) -> Id<HKQuantityType>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other quantity)]
        pub unsafe fn quantity(&self) -> Id<HKQuantity>;

        #[method(count)]
        pub unsafe fn count(&self) -> NSInteger;

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "HealthKit_HKObjectType",
            feature = "HealthKit_HKQuantity"
        ))]
        #[method_id(@__retain_semantics Other quantitySampleWithType:quantity:startDate:endDate:)]
        pub unsafe fn quantitySampleWithType_quantity_startDate_endDate(
            quantity_type: &HKQuantityType,
            quantity: &HKQuantity,
            start_date: &NSDate,
            end_date: &NSDate,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "HealthKit_HKObjectType",
            feature = "HealthKit_HKQuantity"
        ))]
        #[method_id(@__retain_semantics Other quantitySampleWithType:quantity:startDate:endDate:metadata:)]
        pub unsafe fn quantitySampleWithType_quantity_startDate_endDate_metadata(
            quantity_type: &HKQuantityType,
            quantity: &HKQuantity,
            start_date: &NSDate,
            end_date: &NSDate,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "HealthKit_HKDevice",
            feature = "HealthKit_HKObjectType",
            feature = "HealthKit_HKQuantity"
        ))]
        #[method_id(@__retain_semantics Other quantitySampleWithType:quantity:startDate:endDate:device:metadata:)]
        pub unsafe fn quantitySampleWithType_quantity_startDate_endDate_device_metadata(
            quantity_type: &HKQuantityType,
            quantity: &HKQuantity,
            start_date: &NSDate,
            end_date: &NSDate,
            device: Option<&HKDevice>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKObject`
    #[cfg(all(feature = "HealthKit_HKObject", feature = "HealthKit_HKSample"))]
    unsafe impl HKQuantitySample {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "HealthKit_HKObject", feature = "HealthKit_HKSample"))]
    unsafe impl HKQuantitySample {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static HKPredicateKeyPathQuantity: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static HKPredicateKeyPathCount: &'static NSString;
}

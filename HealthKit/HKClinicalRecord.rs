//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkpredicatekeypathclinicalrecordfhirresourceidentifier?language=objc)
    pub static HKPredicateKeyPathClinicalRecordFHIRResourceIdentifier: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkpredicatekeypathclinicalrecordfhirresourcetype?language=objc)
    pub static HKPredicateKeyPathClinicalRecordFHIRResourceType: &'static NSString;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkclinicalrecord?language=objc)
    #[unsafe(super(HKSample, HKObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    pub struct HKClinicalRecord;
);

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl Send for HKClinicalRecord {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl Sync for HKClinicalRecord {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSCoding for HKClinicalRecord {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSCopying for HKClinicalRecord {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl CopyingHelper for HKClinicalRecord {
    type Result = Self;
}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSObjectProtocol for HKClinicalRecord {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSSecureCoding for HKClinicalRecord {}

extern_methods!(
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    unsafe impl HKClinicalRecord {
        #[cfg(all(feature = "HKClinicalType", feature = "HKObjectType"))]
        #[method_id(@__retain_semantics Other clinicalType)]
        pub unsafe fn clinicalType(&self) -> Retained<HKClinicalType>;

        #[method_id(@__retain_semantics Other displayName)]
        pub unsafe fn displayName(&self) -> Retained<NSString>;

        #[cfg(feature = "HKFHIRResource")]
        #[method_id(@__retain_semantics Other FHIRResource)]
        pub unsafe fn FHIRResource(&self) -> Option<Retained<HKFHIRResource>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

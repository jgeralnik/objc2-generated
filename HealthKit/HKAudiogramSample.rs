//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkaudiogramsample?language=objc)
    #[unsafe(super(HKSample, HKObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    pub struct HKAudiogramSample;
);

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl Send for HKAudiogramSample {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl Sync for HKAudiogramSample {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSCoding for HKAudiogramSample {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSObjectProtocol for HKAudiogramSample {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSSecureCoding for HKAudiogramSample {}

extern_methods!(
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    unsafe impl HKAudiogramSample {
        #[cfg(feature = "HKAudiogramSensitivityPoint")]
        #[method_id(@__retain_semantics Other sensitivityPoints)]
        pub unsafe fn sensitivityPoints(&self) -> Retained<NSArray<HKAudiogramSensitivityPoint>>;

        #[cfg(feature = "HKAudiogramSensitivityPoint")]
        #[deprecated]
        #[method_id(@__retain_semantics Other audiogramSampleWithSensitivityPoints:startDate:endDate:metadata:)]
        pub unsafe fn audiogramSampleWithSensitivityPoints_startDate_endDate_metadata(
            sensitivity_points: &NSArray<HKAudiogramSensitivityPoint>,
            start_date: &NSDate,
            end_date: &NSDate,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "HKAudiogramSensitivityPoint", feature = "HKDevice"))]
        #[method_id(@__retain_semantics Other audiogramSampleWithSensitivityPoints:startDate:endDate:device:metadata:)]
        pub unsafe fn audiogramSampleWithSensitivityPoints_startDate_endDate_device_metadata(
            sensitivity_points: &NSArray<HKAudiogramSensitivityPoint>,
            start_date: &NSDate,
            end_date: &NSDate,
            device: Option<&HKDevice>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKObject`
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    unsafe impl HKAudiogramSample {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    unsafe impl HKAudiogramSample {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(MXMetric, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MXMetric")]
    pub struct MXGPUMetric;
);

#[cfg(feature = "MXMetric")]
unsafe impl NSCoding for MXGPUMetric {}

#[cfg(feature = "MXMetric")]
unsafe impl NSObjectProtocol for MXGPUMetric {}

#[cfg(feature = "MXMetric")]
unsafe impl NSSecureCoding for MXGPUMetric {}

extern_methods!(
    #[cfg(feature = "MXMetric")]
    unsafe impl MXGPUMetric {
        #[method_id(@__retain_semantics Other cumulativeGPUTime)]
        pub unsafe fn cumulativeGPUTime(&self) -> Retained<NSMeasurement<NSUnitDuration>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MXMetric")]
    unsafe impl MXGPUMetric {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

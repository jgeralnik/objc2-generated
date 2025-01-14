//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxcpumetric?language=objc)
    #[unsafe(super(MXMetric, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MXMetric")]
    pub struct MXCPUMetric;
);

#[cfg(feature = "MXMetric")]
unsafe impl NSCoding for MXCPUMetric {}

#[cfg(feature = "MXMetric")]
unsafe impl NSObjectProtocol for MXCPUMetric {}

#[cfg(feature = "MXMetric")]
unsafe impl NSSecureCoding for MXCPUMetric {}

extern_methods!(
    #[cfg(feature = "MXMetric")]
    unsafe impl MXCPUMetric {
        #[method_id(@__retain_semantics Other cumulativeCPUTime)]
        pub unsafe fn cumulativeCPUTime(&self) -> Retained<NSMeasurement<NSUnitDuration>>;

        #[method_id(@__retain_semantics Other cumulativeCPUInstructions)]
        pub unsafe fn cumulativeCPUInstructions(&self) -> Retained<NSMeasurement<NSUnit>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MXMetric")]
    unsafe impl MXCPUMetric {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

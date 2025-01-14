//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxcpuexceptiondiagnostic?language=objc)
    #[unsafe(super(MXDiagnostic, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MXDiagnostic")]
    pub struct MXCPUExceptionDiagnostic;
);

#[cfg(feature = "MXDiagnostic")]
unsafe impl NSCoding for MXCPUExceptionDiagnostic {}

#[cfg(feature = "MXDiagnostic")]
unsafe impl NSObjectProtocol for MXCPUExceptionDiagnostic {}

#[cfg(feature = "MXDiagnostic")]
unsafe impl NSSecureCoding for MXCPUExceptionDiagnostic {}

extern_methods!(
    #[cfg(feature = "MXDiagnostic")]
    unsafe impl MXCPUExceptionDiagnostic {
        #[cfg(feature = "MXCallStackTree")]
        #[method_id(@__retain_semantics Other callStackTree)]
        pub unsafe fn callStackTree(&self) -> Retained<MXCallStackTree>;

        #[method_id(@__retain_semantics Other totalCPUTime)]
        pub unsafe fn totalCPUTime(&self) -> Retained<NSMeasurement<NSUnitDuration>>;

        #[method_id(@__retain_semantics Other totalSampledTime)]
        pub unsafe fn totalSampledTime(&self) -> Retained<NSMeasurement<NSUnitDuration>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MXDiagnostic")]
    unsafe impl MXCPUExceptionDiagnostic {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

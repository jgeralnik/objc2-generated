//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxforegroundexitdata?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MXForegroundExitData;
);

unsafe impl NSCoding for MXForegroundExitData {}

unsafe impl NSObjectProtocol for MXForegroundExitData {}

unsafe impl NSSecureCoding for MXForegroundExitData {}

extern_methods!(
    unsafe impl MXForegroundExitData {
        #[method(cumulativeNormalAppExitCount)]
        pub unsafe fn cumulativeNormalAppExitCount(&self) -> NSUInteger;

        #[method(cumulativeMemoryResourceLimitExitCount)]
        pub unsafe fn cumulativeMemoryResourceLimitExitCount(&self) -> NSUInteger;

        #[method(cumulativeBadAccessExitCount)]
        pub unsafe fn cumulativeBadAccessExitCount(&self) -> NSUInteger;

        #[method(cumulativeAbnormalExitCount)]
        pub unsafe fn cumulativeAbnormalExitCount(&self) -> NSUInteger;

        #[method(cumulativeIllegalInstructionExitCount)]
        pub unsafe fn cumulativeIllegalInstructionExitCount(&self) -> NSUInteger;

        #[method(cumulativeAppWatchdogExitCount)]
        pub unsafe fn cumulativeAppWatchdogExitCount(&self) -> NSUInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MXForegroundExitData {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxbackgroundexitdata?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MXBackgroundExitData;
);

unsafe impl NSCoding for MXBackgroundExitData {}

unsafe impl NSObjectProtocol for MXBackgroundExitData {}

unsafe impl NSSecureCoding for MXBackgroundExitData {}

extern_methods!(
    unsafe impl MXBackgroundExitData {
        #[method(cumulativeNormalAppExitCount)]
        pub unsafe fn cumulativeNormalAppExitCount(&self) -> NSUInteger;

        #[method(cumulativeMemoryResourceLimitExitCount)]
        pub unsafe fn cumulativeMemoryResourceLimitExitCount(&self) -> NSUInteger;

        #[method(cumulativeCPUResourceLimitExitCount)]
        pub unsafe fn cumulativeCPUResourceLimitExitCount(&self) -> NSUInteger;

        #[method(cumulativeMemoryPressureExitCount)]
        pub unsafe fn cumulativeMemoryPressureExitCount(&self) -> NSUInteger;

        #[method(cumulativeBadAccessExitCount)]
        pub unsafe fn cumulativeBadAccessExitCount(&self) -> NSUInteger;

        #[method(cumulativeAbnormalExitCount)]
        pub unsafe fn cumulativeAbnormalExitCount(&self) -> NSUInteger;

        #[method(cumulativeIllegalInstructionExitCount)]
        pub unsafe fn cumulativeIllegalInstructionExitCount(&self) -> NSUInteger;

        #[method(cumulativeAppWatchdogExitCount)]
        pub unsafe fn cumulativeAppWatchdogExitCount(&self) -> NSUInteger;

        #[method(cumulativeSuspendedWithLockedFileExitCount)]
        pub unsafe fn cumulativeSuspendedWithLockedFileExitCount(&self) -> NSUInteger;

        #[method(cumulativeBackgroundTaskAssertionTimeoutExitCount)]
        pub unsafe fn cumulativeBackgroundTaskAssertionTimeoutExitCount(&self) -> NSUInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MXBackgroundExitData {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxappexitmetric?language=objc)
    #[unsafe(super(MXMetric, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MXMetric")]
    pub struct MXAppExitMetric;
);

#[cfg(feature = "MXMetric")]
unsafe impl NSCoding for MXAppExitMetric {}

#[cfg(feature = "MXMetric")]
unsafe impl NSObjectProtocol for MXAppExitMetric {}

#[cfg(feature = "MXMetric")]
unsafe impl NSSecureCoding for MXAppExitMetric {}

extern_methods!(
    #[cfg(feature = "MXMetric")]
    unsafe impl MXAppExitMetric {
        #[method_id(@__retain_semantics Other foregroundExitData)]
        pub unsafe fn foregroundExitData(&self) -> Retained<MXForegroundExitData>;

        #[method_id(@__retain_semantics Other backgroundExitData)]
        pub unsafe fn backgroundExitData(&self) -> Retained<MXBackgroundExitData>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MXMetric")]
    unsafe impl MXAppExitMetric {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

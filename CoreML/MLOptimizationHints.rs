//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLOptimizationHints;

    unsafe impl ClassType for MLOptimizationHints {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for MLOptimizationHints {}

unsafe impl NSCopying for MLOptimizationHints {}

unsafe impl CopyingHelper for MLOptimizationHints {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MLOptimizationHints {}

unsafe impl NSSecureCoding for MLOptimizationHints {}

extern_methods!(
    unsafe impl MLOptimizationHints {
        #[cfg(feature = "MLReshapeFrequencyHint")]
        #[method(reshapeFrequency)]
        pub unsafe fn reshapeFrequency(&self) -> MLReshapeFrequencyHint;

        #[cfg(feature = "MLReshapeFrequencyHint")]
        #[method(setReshapeFrequency:)]
        pub unsafe fn setReshapeFrequency(&self, reshape_frequency: MLReshapeFrequencyHint);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MLOptimizationHints {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

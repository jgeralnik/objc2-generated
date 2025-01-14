//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcoptimizer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct MLCOptimizer;
);

unsafe impl NSCopying for MLCOptimizer {}

unsafe impl CopyingHelper for MLCOptimizer {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MLCOptimizer {}

extern_methods!(
    unsafe impl MLCOptimizer {
        #[deprecated]
        #[method(learningRate)]
        pub unsafe fn learningRate(&self) -> c_float;

        #[deprecated]
        #[method(setLearningRate:)]
        pub unsafe fn setLearningRate(&self, learning_rate: c_float);

        #[deprecated]
        #[method(gradientRescale)]
        pub unsafe fn gradientRescale(&self) -> c_float;

        #[deprecated]
        #[method(appliesGradientClipping)]
        pub unsafe fn appliesGradientClipping(&self) -> bool;

        #[deprecated]
        #[method(setAppliesGradientClipping:)]
        pub unsafe fn setAppliesGradientClipping(&self, applies_gradient_clipping: bool);

        #[deprecated]
        #[method(gradientClipMax)]
        pub unsafe fn gradientClipMax(&self) -> c_float;

        #[deprecated]
        #[method(gradientClipMin)]
        pub unsafe fn gradientClipMin(&self) -> c_float;

        #[deprecated]
        #[method(regularizationScale)]
        pub unsafe fn regularizationScale(&self) -> c_float;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method(regularizationType)]
        pub unsafe fn regularizationType(&self) -> MLCRegularizationType;

        #[cfg(feature = "MLCTypes")]
        #[method(gradientClippingType)]
        pub unsafe fn gradientClippingType(&self) -> MLCGradientClippingType;

        #[method(maximumClippingNorm)]
        pub unsafe fn maximumClippingNorm(&self) -> c_float;

        #[method(customGlobalNorm)]
        pub unsafe fn customGlobalNorm(&self) -> c_float;

        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

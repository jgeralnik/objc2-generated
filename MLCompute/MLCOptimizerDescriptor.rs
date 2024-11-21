//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct MLCOptimizerDescriptor;
);

unsafe impl NSCopying for MLCOptimizerDescriptor {}

unsafe impl CopyingHelper for MLCOptimizerDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MLCOptimizerDescriptor {}

extern_methods!(
    unsafe impl MLCOptimizerDescriptor {
        #[deprecated]
        #[method(learningRate)]
        pub unsafe fn learningRate(&self) -> c_float;

        #[deprecated]
        #[method(gradientRescale)]
        pub unsafe fn gradientRescale(&self) -> c_float;

        #[deprecated]
        #[method(appliesGradientClipping)]
        pub unsafe fn appliesGradientClipping(&self) -> bool;

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

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptorWithLearningRate:gradientRescale:regularizationType:regularizationScale:)]
        pub unsafe fn descriptorWithLearningRate_gradientRescale_regularizationType_regularizationScale(
            learning_rate: c_float,
            gradient_rescale: c_float,
            regularization_type: MLCRegularizationType,
            regularization_scale: c_float,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptorWithLearningRate:gradientRescale:appliesGradientClipping:gradientClipMax:gradientClipMin:regularizationType:regularizationScale:)]
        pub unsafe fn descriptorWithLearningRate_gradientRescale_appliesGradientClipping_gradientClipMax_gradientClipMin_regularizationType_regularizationScale(
            learning_rate: c_float,
            gradient_rescale: c_float,
            applies_gradient_clipping: bool,
            gradient_clip_max: c_float,
            gradient_clip_min: c_float,
            regularization_type: MLCRegularizationType,
            regularization_scale: c_float,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[method_id(@__retain_semantics Other descriptorWithLearningRate:gradientRescale:appliesGradientClipping:gradientClippingType:gradientClipMax:gradientClipMin:maximumClippingNorm:customGlobalNorm:regularizationType:regularizationScale:)]
        pub unsafe fn descriptorWithLearningRate_gradientRescale_appliesGradientClipping_gradientClippingType_gradientClipMax_gradientClipMin_maximumClippingNorm_customGlobalNorm_regularizationType_regularizationScale(
            learning_rate: c_float,
            gradient_rescale: c_float,
            applies_gradient_clipping: bool,
            gradient_clipping_type: MLCGradientClippingType,
            gradient_clip_max: c_float,
            gradient_clip_min: c_float,
            maximum_clipping_norm: c_float,
            custom_global_norm: c_float,
            regularization_type: MLCRegularizationType,
            regularization_scale: c_float,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MLCOptimizerDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

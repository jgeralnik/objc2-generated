//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcrmspropoptimizer?language=objc)
    #[unsafe(super(MLCOptimizer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCOptimizer")]
    #[deprecated = "Use Metal Performance Shaders Graph or BNNS instead."]
    pub struct MLCRMSPropOptimizer;
);

#[cfg(feature = "MLCOptimizer")]
unsafe impl NSCopying for MLCRMSPropOptimizer {}

#[cfg(feature = "MLCOptimizer")]
unsafe impl CopyingHelper for MLCRMSPropOptimizer {
    type Result = Self;
}

#[cfg(feature = "MLCOptimizer")]
unsafe impl NSObjectProtocol for MLCRMSPropOptimizer {}

extern_methods!(
    #[cfg(feature = "MLCOptimizer")]
    unsafe impl MLCRMSPropOptimizer {
        #[deprecated = "Use Metal Performance Shaders Graph or BNNS instead."]
        #[method(momentumScale)]
        pub unsafe fn momentumScale(&self) -> c_float;

        #[deprecated = "Use Metal Performance Shaders Graph or BNNS instead."]
        #[method(alpha)]
        pub unsafe fn alpha(&self) -> c_float;

        #[deprecated = "Use Metal Performance Shaders Graph or BNNS instead."]
        #[method(epsilon)]
        pub unsafe fn epsilon(&self) -> c_float;

        #[deprecated = "Use Metal Performance Shaders Graph or BNNS instead."]
        #[method(isCentered)]
        pub unsafe fn isCentered(&self) -> bool;

        #[cfg(feature = "MLCOptimizerDescriptor")]
        #[deprecated = "Use Metal Performance Shaders Graph or BNNS instead."]
        #[method_id(@__retain_semantics Other optimizerWithDescriptor:)]
        pub unsafe fn optimizerWithDescriptor(
            optimizer_descriptor: &MLCOptimizerDescriptor,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCOptimizerDescriptor")]
        #[deprecated = "Use Metal Performance Shaders Graph or BNNS instead."]
        #[method_id(@__retain_semantics Other optimizerWithDescriptor:momentumScale:alpha:epsilon:isCentered:)]
        pub unsafe fn optimizerWithDescriptor_momentumScale_alpha_epsilon_isCentered(
            optimizer_descriptor: &MLCOptimizerDescriptor,
            momentum_scale: c_float,
            alpha: c_float,
            epsilon: c_float,
            is_centered: bool,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCOptimizer`
    #[cfg(feature = "MLCOptimizer")]
    unsafe impl MLCRMSPropOptimizer {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

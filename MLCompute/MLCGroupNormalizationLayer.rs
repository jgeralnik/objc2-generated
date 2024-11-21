//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[unsafe(super(MLCLayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCLayer")]
    #[deprecated]
    pub struct MLCGroupNormalizationLayer;
);

#[cfg(feature = "MLCLayer")]
unsafe impl NSObjectProtocol for MLCGroupNormalizationLayer {}

extern_methods!(
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCGroupNormalizationLayer {
        #[deprecated]
        #[method(featureChannelCount)]
        pub unsafe fn featureChannelCount(&self) -> NSUInteger;

        #[deprecated]
        #[method(groupCount)]
        pub unsafe fn groupCount(&self) -> NSUInteger;

        #[cfg(feature = "MLCTensor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other beta)]
        pub unsafe fn beta(&self) -> Option<Retained<MLCTensor>>;

        #[cfg(feature = "MLCTensor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other gamma)]
        pub unsafe fn gamma(&self) -> Option<Retained<MLCTensor>>;

        #[cfg(feature = "MLCTensorParameter")]
        #[deprecated]
        #[method_id(@__retain_semantics Other betaParameter)]
        pub unsafe fn betaParameter(&self) -> Option<Retained<MLCTensorParameter>>;

        #[cfg(feature = "MLCTensorParameter")]
        #[deprecated]
        #[method_id(@__retain_semantics Other gammaParameter)]
        pub unsafe fn gammaParameter(&self) -> Option<Retained<MLCTensorParameter>>;

        #[deprecated]
        #[method(varianceEpsilon)]
        pub unsafe fn varianceEpsilon(&self) -> c_float;

        #[cfg(feature = "MLCTensor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithFeatureChannelCount:groupCount:beta:gamma:varianceEpsilon:)]
        pub unsafe fn layerWithFeatureChannelCount_groupCount_beta_gamma_varianceEpsilon(
            feature_channel_count: NSUInteger,
            group_count: NSUInteger,
            beta: Option<&MLCTensor>,
            gamma: Option<&MLCTensor>,
            variance_epsilon: c_float,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCGroupNormalizationLayer {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

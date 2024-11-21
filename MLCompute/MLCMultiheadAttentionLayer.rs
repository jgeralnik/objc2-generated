//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(MLCLayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCLayer")]
    #[deprecated]
    pub struct MLCMultiheadAttentionLayer;
);

#[cfg(feature = "MLCLayer")]
unsafe impl NSObjectProtocol for MLCMultiheadAttentionLayer {}

extern_methods!(
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCMultiheadAttentionLayer {
        #[cfg(feature = "MLCMultiheadAttentionDescriptor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptor)]
        pub unsafe fn descriptor(&self) -> Retained<MLCMultiheadAttentionDescriptor>;

        #[cfg(feature = "MLCTensor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other weights)]
        pub unsafe fn weights(&self) -> Retained<NSArray<MLCTensor>>;

        #[cfg(feature = "MLCTensor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other biases)]
        pub unsafe fn biases(&self) -> Option<Retained<NSArray<MLCTensor>>>;

        #[cfg(feature = "MLCTensor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other attentionBiases)]
        pub unsafe fn attentionBiases(&self) -> Option<Retained<NSArray<MLCTensor>>>;

        #[cfg(feature = "MLCTensorParameter")]
        #[deprecated]
        #[method_id(@__retain_semantics Other weightsParameters)]
        pub unsafe fn weightsParameters(&self) -> Retained<NSArray<MLCTensorParameter>>;

        #[cfg(feature = "MLCTensorParameter")]
        #[deprecated]
        #[method_id(@__retain_semantics Other biasesParameters)]
        pub unsafe fn biasesParameters(&self) -> Option<Retained<NSArray<MLCTensorParameter>>>;

        #[cfg(all(feature = "MLCMultiheadAttentionDescriptor", feature = "MLCTensor"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithDescriptor:weights:biases:attentionBiases:)]
        pub unsafe fn layerWithDescriptor_weights_biases_attentionBiases(
            descriptor: &MLCMultiheadAttentionDescriptor,
            weights: &NSArray<MLCTensor>,
            biases: Option<&NSArray<MLCTensor>>,
            attention_biases: Option<&NSArray<MLCTensor>>,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCMultiheadAttentionLayer {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

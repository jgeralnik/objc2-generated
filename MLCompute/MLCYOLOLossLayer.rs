//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MLCLayer", feature = "MLCLossLayer"))]
    #[deprecated]
    pub struct MLCYOLOLossLayer;

    #[cfg(all(feature = "MLCLayer", feature = "MLCLossLayer"))]
    unsafe impl ClassType for MLCYOLOLossLayer {
        #[inherits(MLCLayer, NSObject)]
        type Super = MLCLossLayer;
    }
);

#[cfg(all(feature = "MLCLayer", feature = "MLCLossLayer"))]
unsafe impl NSObjectProtocol for MLCYOLOLossLayer {}

extern_methods!(
    #[cfg(all(feature = "MLCLayer", feature = "MLCLossLayer"))]
    unsafe impl MLCYOLOLossLayer {
        #[cfg(feature = "MLCYOLOLossDescriptor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other yoloLossDescriptor)]
        pub unsafe fn yoloLossDescriptor(&self) -> Retained<MLCYOLOLossDescriptor>;

        #[cfg(feature = "MLCYOLOLossDescriptor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithDescriptor:)]
        pub unsafe fn layerWithDescriptor(
            loss_descriptor: &MLCYOLOLossDescriptor,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLossLayer`
    #[cfg(all(feature = "MLCLayer", feature = "MLCLossLayer"))]
    unsafe impl MLCYOLOLossLayer {
        #[cfg(all(feature = "MLCLossDescriptor", feature = "MLCTensor"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithDescriptor:weights:)]
        pub unsafe fn layerWithDescriptor_weights(
            loss_descriptor: &MLCLossDescriptor,
            weights: &MLCTensor,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other softmaxCrossEntropyLossWithReductionType:labelSmoothing:classCount:weight:)]
        pub unsafe fn softmaxCrossEntropyLossWithReductionType_labelSmoothing_classCount_weight(
            reduction_type: MLCReductionType,
            label_smoothing: c_float,
            class_count: NSUInteger,
            weight: c_float,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other softmaxCrossEntropyLossWithReductionType:labelSmoothing:classCount:weights:)]
        pub unsafe fn softmaxCrossEntropyLossWithReductionType_labelSmoothing_classCount_weights(
            reduction_type: MLCReductionType,
            label_smoothing: c_float,
            class_count: NSUInteger,
            weights: Option<&MLCTensor>,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other categoricalCrossEntropyLossWithReductionType:labelSmoothing:classCount:weight:)]
        pub unsafe fn categoricalCrossEntropyLossWithReductionType_labelSmoothing_classCount_weight(
            reduction_type: MLCReductionType,
            label_smoothing: c_float,
            class_count: NSUInteger,
            weight: c_float,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other categoricalCrossEntropyLossWithReductionType:labelSmoothing:classCount:weights:)]
        pub unsafe fn categoricalCrossEntropyLossWithReductionType_labelSmoothing_classCount_weights(
            reduction_type: MLCReductionType,
            label_smoothing: c_float,
            class_count: NSUInteger,
            weights: Option<&MLCTensor>,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other sigmoidCrossEntropyLossWithReductionType:labelSmoothing:weight:)]
        pub unsafe fn sigmoidCrossEntropyLossWithReductionType_labelSmoothing_weight(
            reduction_type: MLCReductionType,
            label_smoothing: c_float,
            weight: c_float,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other sigmoidCrossEntropyLossWithReductionType:labelSmoothing:weights:)]
        pub unsafe fn sigmoidCrossEntropyLossWithReductionType_labelSmoothing_weights(
            reduction_type: MLCReductionType,
            label_smoothing: c_float,
            weights: Option<&MLCTensor>,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other logLossWithReductionType:epsilon:weight:)]
        pub unsafe fn logLossWithReductionType_epsilon_weight(
            reduction_type: MLCReductionType,
            epsilon: c_float,
            weight: c_float,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other logLossWithReductionType:epsilon:weights:)]
        pub unsafe fn logLossWithReductionType_epsilon_weights(
            reduction_type: MLCReductionType,
            epsilon: c_float,
            weights: Option<&MLCTensor>,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other huberLossWithReductionType:delta:weight:)]
        pub unsafe fn huberLossWithReductionType_delta_weight(
            reduction_type: MLCReductionType,
            delta: c_float,
            weight: c_float,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other huberLossWithReductionType:delta:weights:)]
        pub unsafe fn huberLossWithReductionType_delta_weights(
            reduction_type: MLCReductionType,
            delta: c_float,
            weights: Option<&MLCTensor>,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other meanAbsoluteErrorLossWithReductionType:weight:)]
        pub unsafe fn meanAbsoluteErrorLossWithReductionType_weight(
            reduction_type: MLCReductionType,
            weight: c_float,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other meanAbsoluteErrorLossWithReductionType:weights:)]
        pub unsafe fn meanAbsoluteErrorLossWithReductionType_weights(
            reduction_type: MLCReductionType,
            weights: Option<&MLCTensor>,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other meanSquaredErrorLossWithReductionType:weight:)]
        pub unsafe fn meanSquaredErrorLossWithReductionType_weight(
            reduction_type: MLCReductionType,
            weight: c_float,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other meanSquaredErrorLossWithReductionType:weights:)]
        pub unsafe fn meanSquaredErrorLossWithReductionType_weights(
            reduction_type: MLCReductionType,
            weights: Option<&MLCTensor>,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other hingeLossWithReductionType:weight:)]
        pub unsafe fn hingeLossWithReductionType_weight(
            reduction_type: MLCReductionType,
            weight: c_float,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other hingeLossWithReductionType:weights:)]
        pub unsafe fn hingeLossWithReductionType_weights(
            reduction_type: MLCReductionType,
            weights: Option<&MLCTensor>,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other cosineDistanceLossWithReductionType:weight:)]
        pub unsafe fn cosineDistanceLossWithReductionType_weight(
            reduction_type: MLCReductionType,
            weight: c_float,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other cosineDistanceLossWithReductionType:weights:)]
        pub unsafe fn cosineDistanceLossWithReductionType_weights(
            reduction_type: MLCReductionType,
            weights: Option<&MLCTensor>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(all(feature = "MLCLayer", feature = "MLCLossLayer"))]
    unsafe impl MLCYOLOLossLayer {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

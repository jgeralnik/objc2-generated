// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]

#[link(name = "MLCompute", kind = "framework")]
extern "C" {}

#[cfg(feature = "MLCActivationDescriptor")]
#[path = "MLCActivationDescriptor.rs"]
mod __MLCActivationDescriptor;
#[cfg(feature = "MLCActivationLayer")]
#[path = "MLCActivationLayer.rs"]
mod __MLCActivationLayer;
#[cfg(feature = "MLCAdamOptimizer")]
#[path = "MLCAdamOptimizer.rs"]
mod __MLCAdamOptimizer;
#[cfg(feature = "MLCAdamWOptimizer")]
#[path = "MLCAdamWOptimizer.rs"]
mod __MLCAdamWOptimizer;
#[cfg(feature = "MLCArithmeticLayer")]
#[path = "MLCArithmeticLayer.rs"]
mod __MLCArithmeticLayer;
#[cfg(feature = "MLCBatchNormalizationLayer")]
#[path = "MLCBatchNormalizationLayer.rs"]
mod __MLCBatchNormalizationLayer;
#[cfg(feature = "MLCComparisonLayer")]
#[path = "MLCComparisonLayer.rs"]
mod __MLCComparisonLayer;
#[cfg(feature = "MLCConcatenationLayer")]
#[path = "MLCConcatenationLayer.rs"]
mod __MLCConcatenationLayer;
#[cfg(feature = "MLCConvolutionDescriptor")]
#[path = "MLCConvolutionDescriptor.rs"]
mod __MLCConvolutionDescriptor;
#[cfg(feature = "MLCConvolutionLayer")]
#[path = "MLCConvolutionLayer.rs"]
mod __MLCConvolutionLayer;
#[cfg(feature = "MLCDefines")]
#[path = "MLCDefines.rs"]
mod __MLCDefines;
#[cfg(feature = "MLCDevice")]
#[path = "MLCDevice.rs"]
mod __MLCDevice;
#[cfg(feature = "MLCDropoutLayer")]
#[path = "MLCDropoutLayer.rs"]
mod __MLCDropoutLayer;
#[cfg(feature = "MLCEmbeddingDescriptor")]
#[path = "MLCEmbeddingDescriptor.rs"]
mod __MLCEmbeddingDescriptor;
#[cfg(feature = "MLCEmbeddingLayer")]
#[path = "MLCEmbeddingLayer.rs"]
mod __MLCEmbeddingLayer;
#[cfg(feature = "MLCFullyConnectedLayer")]
#[path = "MLCFullyConnectedLayer.rs"]
mod __MLCFullyConnectedLayer;
#[cfg(feature = "MLCGatherLayer")]
#[path = "MLCGatherLayer.rs"]
mod __MLCGatherLayer;
#[cfg(feature = "MLCGramMatrixLayer")]
#[path = "MLCGramMatrixLayer.rs"]
mod __MLCGramMatrixLayer;
#[cfg(feature = "MLCGraph")]
#[path = "MLCGraph.rs"]
mod __MLCGraph;
#[cfg(feature = "MLCGroupNormalizationLayer")]
#[path = "MLCGroupNormalizationLayer.rs"]
mod __MLCGroupNormalizationLayer;
#[cfg(feature = "MLCInferenceGraph")]
#[path = "MLCInferenceGraph.rs"]
mod __MLCInferenceGraph;
#[cfg(feature = "MLCInstanceNormalizationLayer")]
#[path = "MLCInstanceNormalizationLayer.rs"]
mod __MLCInstanceNormalizationLayer;
#[cfg(feature = "MLCLSTMDescriptor")]
#[path = "MLCLSTMDescriptor.rs"]
mod __MLCLSTMDescriptor;
#[cfg(feature = "MLCLSTMLayer")]
#[path = "MLCLSTMLayer.rs"]
mod __MLCLSTMLayer;
#[cfg(feature = "MLCLayer")]
#[path = "MLCLayer.rs"]
mod __MLCLayer;
#[cfg(feature = "MLCLayerNormalizationLayer")]
#[path = "MLCLayerNormalizationLayer.rs"]
mod __MLCLayerNormalizationLayer;
#[cfg(feature = "MLCLossDescriptor")]
#[path = "MLCLossDescriptor.rs"]
mod __MLCLossDescriptor;
#[cfg(feature = "MLCLossLayer")]
#[path = "MLCLossLayer.rs"]
mod __MLCLossLayer;
#[cfg(feature = "MLCMatMulDescriptor")]
#[path = "MLCMatMulDescriptor.rs"]
mod __MLCMatMulDescriptor;
#[cfg(feature = "MLCMatMulLayer")]
#[path = "MLCMatMulLayer.rs"]
mod __MLCMatMulLayer;
#[cfg(feature = "MLCMultiheadAttentionDescriptor")]
#[path = "MLCMultiheadAttentionDescriptor.rs"]
mod __MLCMultiheadAttentionDescriptor;
#[cfg(feature = "MLCMultiheadAttentionLayer")]
#[path = "MLCMultiheadAttentionLayer.rs"]
mod __MLCMultiheadAttentionLayer;
#[cfg(feature = "MLCOptimizer")]
#[path = "MLCOptimizer.rs"]
mod __MLCOptimizer;
#[cfg(feature = "MLCOptimizerDescriptor")]
#[path = "MLCOptimizerDescriptor.rs"]
mod __MLCOptimizerDescriptor;
#[cfg(feature = "MLCPaddingLayer")]
#[path = "MLCPaddingLayer.rs"]
mod __MLCPaddingLayer;
#[cfg(feature = "MLCPlatform")]
#[path = "MLCPlatform.rs"]
mod __MLCPlatform;
#[cfg(feature = "MLCPoolingDescriptor")]
#[path = "MLCPoolingDescriptor.rs"]
mod __MLCPoolingDescriptor;
#[cfg(feature = "MLCPoolingLayer")]
#[path = "MLCPoolingLayer.rs"]
mod __MLCPoolingLayer;
#[cfg(feature = "MLCRMSPropOptimizer")]
#[path = "MLCRMSPropOptimizer.rs"]
mod __MLCRMSPropOptimizer;
#[cfg(feature = "MLCReductionLayer")]
#[path = "MLCReductionLayer.rs"]
mod __MLCReductionLayer;
#[cfg(feature = "MLCReshapeLayer")]
#[path = "MLCReshapeLayer.rs"]
mod __MLCReshapeLayer;
#[cfg(feature = "MLCSGDOptimizer")]
#[path = "MLCSGDOptimizer.rs"]
mod __MLCSGDOptimizer;
#[cfg(feature = "MLCScatterLayer")]
#[path = "MLCScatterLayer.rs"]
mod __MLCScatterLayer;
#[cfg(feature = "MLCSelectionLayer")]
#[path = "MLCSelectionLayer.rs"]
mod __MLCSelectionLayer;
#[cfg(feature = "MLCSliceLayer")]
#[path = "MLCSliceLayer.rs"]
mod __MLCSliceLayer;
#[cfg(feature = "MLCSoftmaxLayer")]
#[path = "MLCSoftmaxLayer.rs"]
mod __MLCSoftmaxLayer;
#[cfg(feature = "MLCSplitLayer")]
#[path = "MLCSplitLayer.rs"]
mod __MLCSplitLayer;
#[cfg(feature = "MLCTensor")]
#[path = "MLCTensor.rs"]
mod __MLCTensor;
#[cfg(feature = "MLCTensorData")]
#[path = "MLCTensorData.rs"]
mod __MLCTensorData;
#[cfg(feature = "MLCTensorDescriptor")]
#[path = "MLCTensorDescriptor.rs"]
mod __MLCTensorDescriptor;
#[cfg(feature = "MLCTensorOptimizerDeviceData")]
#[path = "MLCTensorOptimizerDeviceData.rs"]
mod __MLCTensorOptimizerDeviceData;
#[cfg(feature = "MLCTensorParameter")]
#[path = "MLCTensorParameter.rs"]
mod __MLCTensorParameter;
#[cfg(feature = "MLCTrainingGraph")]
#[path = "MLCTrainingGraph.rs"]
mod __MLCTrainingGraph;
#[cfg(feature = "MLCTransposeLayer")]
#[path = "MLCTransposeLayer.rs"]
mod __MLCTransposeLayer;
#[cfg(feature = "MLCTypes")]
#[path = "MLCTypes.rs"]
mod __MLCTypes;
#[cfg(feature = "MLCUpsampleLayer")]
#[path = "MLCUpsampleLayer.rs"]
mod __MLCUpsampleLayer;
#[cfg(feature = "MLCYOLOLossDescriptor")]
#[path = "MLCYOLOLossDescriptor.rs"]
mod __MLCYOLOLossDescriptor;
#[cfg(feature = "MLCYOLOLossLayer")]
#[path = "MLCYOLOLossLayer.rs"]
mod __MLCYOLOLossLayer;

#[cfg(feature = "MLCActivationDescriptor")]
pub use self::__MLCActivationDescriptor::MLCActivationDescriptor;
#[cfg(all(feature = "MLCActivationLayer", feature = "MLCLayer"))]
pub use self::__MLCActivationLayer::MLCActivationLayer;
#[cfg(all(feature = "MLCAdamOptimizer", feature = "MLCOptimizer"))]
pub use self::__MLCAdamOptimizer::MLCAdamOptimizer;
#[cfg(all(feature = "MLCAdamWOptimizer", feature = "MLCOptimizer"))]
pub use self::__MLCAdamWOptimizer::MLCAdamWOptimizer;
#[cfg(all(feature = "MLCArithmeticLayer", feature = "MLCLayer"))]
pub use self::__MLCArithmeticLayer::MLCArithmeticLayer;
#[cfg(all(feature = "MLCBatchNormalizationLayer", feature = "MLCLayer"))]
pub use self::__MLCBatchNormalizationLayer::MLCBatchNormalizationLayer;
#[cfg(all(feature = "MLCComparisonLayer", feature = "MLCLayer"))]
pub use self::__MLCComparisonLayer::MLCComparisonLayer;
#[cfg(all(feature = "MLCConcatenationLayer", feature = "MLCLayer"))]
pub use self::__MLCConcatenationLayer::MLCConcatenationLayer;
#[cfg(feature = "MLCConvolutionDescriptor")]
pub use self::__MLCConvolutionDescriptor::MLCConvolutionDescriptor;
#[cfg(all(feature = "MLCConvolutionLayer", feature = "MLCLayer"))]
pub use self::__MLCConvolutionLayer::MLCConvolutionLayer;
#[cfg(feature = "MLCDevice")]
pub use self::__MLCDevice::MLCDevice;
#[cfg(all(feature = "MLCDropoutLayer", feature = "MLCLayer"))]
pub use self::__MLCDropoutLayer::MLCDropoutLayer;
#[cfg(feature = "MLCEmbeddingDescriptor")]
pub use self::__MLCEmbeddingDescriptor::MLCEmbeddingDescriptor;
#[cfg(all(feature = "MLCEmbeddingLayer", feature = "MLCLayer"))]
pub use self::__MLCEmbeddingLayer::MLCEmbeddingLayer;
#[cfg(all(feature = "MLCFullyConnectedLayer", feature = "MLCLayer"))]
pub use self::__MLCFullyConnectedLayer::MLCFullyConnectedLayer;
#[cfg(all(feature = "MLCGatherLayer", feature = "MLCLayer"))]
pub use self::__MLCGatherLayer::MLCGatherLayer;
#[cfg(all(feature = "MLCGramMatrixLayer", feature = "MLCLayer"))]
pub use self::__MLCGramMatrixLayer::MLCGramMatrixLayer;
#[cfg(feature = "MLCGraph")]
pub use self::__MLCGraph::MLCGraph;
#[cfg(all(feature = "MLCGroupNormalizationLayer", feature = "MLCLayer"))]
pub use self::__MLCGroupNormalizationLayer::MLCGroupNormalizationLayer;
#[cfg(all(feature = "MLCGraph", feature = "MLCInferenceGraph"))]
pub use self::__MLCInferenceGraph::MLCInferenceGraph;
#[cfg(all(feature = "MLCInstanceNormalizationLayer", feature = "MLCLayer"))]
pub use self::__MLCInstanceNormalizationLayer::MLCInstanceNormalizationLayer;
#[cfg(feature = "MLCLSTMDescriptor")]
pub use self::__MLCLSTMDescriptor::MLCLSTMDescriptor;
#[cfg(all(feature = "MLCLSTMLayer", feature = "MLCLayer"))]
pub use self::__MLCLSTMLayer::MLCLSTMLayer;
#[cfg(feature = "MLCLayer")]
pub use self::__MLCLayer::MLCLayer;
#[cfg(all(feature = "MLCLayer", feature = "MLCLayerNormalizationLayer"))]
pub use self::__MLCLayerNormalizationLayer::MLCLayerNormalizationLayer;
#[cfg(feature = "MLCLossDescriptor")]
pub use self::__MLCLossDescriptor::MLCLossDescriptor;
#[cfg(all(feature = "MLCLayer", feature = "MLCLossLayer"))]
pub use self::__MLCLossLayer::MLCLossLayer;
#[cfg(feature = "MLCMatMulDescriptor")]
pub use self::__MLCMatMulDescriptor::MLCMatMulDescriptor;
#[cfg(all(feature = "MLCLayer", feature = "MLCMatMulLayer"))]
pub use self::__MLCMatMulLayer::MLCMatMulLayer;
#[cfg(feature = "MLCMultiheadAttentionDescriptor")]
pub use self::__MLCMultiheadAttentionDescriptor::MLCMultiheadAttentionDescriptor;
#[cfg(all(feature = "MLCLayer", feature = "MLCMultiheadAttentionLayer"))]
pub use self::__MLCMultiheadAttentionLayer::MLCMultiheadAttentionLayer;
#[cfg(feature = "MLCOptimizer")]
pub use self::__MLCOptimizer::MLCOptimizer;
#[cfg(feature = "MLCOptimizerDescriptor")]
pub use self::__MLCOptimizerDescriptor::MLCOptimizerDescriptor;
#[cfg(all(feature = "MLCLayer", feature = "MLCPaddingLayer"))]
pub use self::__MLCPaddingLayer::MLCPaddingLayer;
#[cfg(feature = "MLCPlatform")]
pub use self::__MLCPlatform::MLCPlatform;
#[cfg(feature = "MLCPoolingDescriptor")]
pub use self::__MLCPoolingDescriptor::MLCPoolingDescriptor;
#[cfg(all(feature = "MLCLayer", feature = "MLCPoolingLayer"))]
pub use self::__MLCPoolingLayer::MLCPoolingLayer;
#[cfg(all(feature = "MLCOptimizer", feature = "MLCRMSPropOptimizer"))]
pub use self::__MLCRMSPropOptimizer::MLCRMSPropOptimizer;
#[cfg(all(feature = "MLCLayer", feature = "MLCReductionLayer"))]
pub use self::__MLCReductionLayer::MLCReductionLayer;
#[cfg(all(feature = "MLCLayer", feature = "MLCReshapeLayer"))]
pub use self::__MLCReshapeLayer::MLCReshapeLayer;
#[cfg(all(feature = "MLCOptimizer", feature = "MLCSGDOptimizer"))]
pub use self::__MLCSGDOptimizer::MLCSGDOptimizer;
#[cfg(all(feature = "MLCLayer", feature = "MLCScatterLayer"))]
pub use self::__MLCScatterLayer::MLCScatterLayer;
#[cfg(all(feature = "MLCLayer", feature = "MLCSelectionLayer"))]
pub use self::__MLCSelectionLayer::MLCSelectionLayer;
#[cfg(all(feature = "MLCLayer", feature = "MLCSliceLayer"))]
pub use self::__MLCSliceLayer::MLCSliceLayer;
#[cfg(all(feature = "MLCLayer", feature = "MLCSoftmaxLayer"))]
pub use self::__MLCSoftmaxLayer::MLCSoftmaxLayer;
#[cfg(all(feature = "MLCLayer", feature = "MLCSplitLayer"))]
pub use self::__MLCSplitLayer::MLCSplitLayer;
#[cfg(feature = "MLCTensor")]
pub use self::__MLCTensor::MLCTensor;
#[cfg(feature = "MLCTensorData")]
pub use self::__MLCTensorData::MLCTensorData;
#[cfg(feature = "MLCTensorDescriptor")]
pub use self::__MLCTensorDescriptor::MLCTensorDescriptor;
#[cfg(feature = "MLCTensorOptimizerDeviceData")]
pub use self::__MLCTensorOptimizerDeviceData::MLCTensorOptimizerDeviceData;
#[cfg(feature = "MLCTensorParameter")]
pub use self::__MLCTensorParameter::MLCTensorParameter;
#[cfg(all(feature = "MLCGraph", feature = "MLCTrainingGraph"))]
pub use self::__MLCTrainingGraph::MLCTrainingGraph;
#[cfg(all(feature = "MLCLayer", feature = "MLCTransposeLayer"))]
pub use self::__MLCTransposeLayer::MLCTransposeLayer;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCActivationType;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCActivationTypeDebugDescription;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCArithmeticOperation;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCArithmeticOperationDebugDescription;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCComparisonOperation;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCComparisonOperationDebugDescription;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCConvolutionType;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCConvolutionTypeDebugDescription;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCDataType;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCDeviceType;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCExecutionOptions;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCGradientClippingType;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCGradientClippingTypeDebugDescription;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCGraphCompilationOptions;
#[cfg(all(feature = "MLCTensor", feature = "MLCTypes", feature = "block2"))]
pub use self::__MLCTypes::MLCGraphCompletionHandler;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCLSTMResultMode;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCLSTMResultModeDebugDescription;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCLossType;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCLossTypeDebugDescription;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCPaddingPolicy;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCPaddingPolicyDebugDescription;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCPaddingType;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCPaddingTypeDebugDescription;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCPoolingType;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCPoolingTypeDebugDescription;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCRandomInitializerType;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCReductionType;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCReductionTypeDebugDescription;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCRegularizationType;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCSampleMode;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCSampleModeDebugDescription;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCSoftmaxOperation;
#[cfg(feature = "MLCTypes")]
pub use self::__MLCTypes::MLCSoftmaxOperationDebugDescription;
#[cfg(all(feature = "MLCLayer", feature = "MLCUpsampleLayer"))]
pub use self::__MLCUpsampleLayer::MLCUpsampleLayer;
#[cfg(feature = "MLCYOLOLossDescriptor")]
pub use self::__MLCYOLOLossDescriptor::MLCYOLOLossDescriptor;
#[cfg(all(
    feature = "MLCLayer",
    feature = "MLCLossLayer",
    feature = "MLCYOLOLossLayer"
))]
pub use self::__MLCYOLOLossLayer::MLCYOLOLossLayer;
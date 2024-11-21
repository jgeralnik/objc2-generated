//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct MLCTensorDescriptor;
);

unsafe impl NSCopying for MLCTensorDescriptor {}

unsafe impl CopyingHelper for MLCTensorDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MLCTensorDescriptor {}

extern_methods!(
    unsafe impl MLCTensorDescriptor {
        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method(dataType)]
        pub unsafe fn dataType(&self) -> MLCDataType;

        #[deprecated]
        #[method(dimensionCount)]
        pub unsafe fn dimensionCount(&self) -> NSUInteger;

        #[deprecated]
        #[method_id(@__retain_semantics Other shape)]
        pub unsafe fn shape(&self) -> Retained<NSArray<NSNumber>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other stride)]
        pub unsafe fn stride(&self) -> Retained<NSArray<NSNumber>>;

        #[deprecated]
        #[method(tensorAllocationSizeInBytes)]
        pub unsafe fn tensorAllocationSizeInBytes(&self) -> NSUInteger;

        #[deprecated]
        #[method_id(@__retain_semantics Other sequenceLengths)]
        pub unsafe fn sequenceLengths(&self) -> Option<Retained<NSArray<NSNumber>>>;

        #[deprecated]
        #[method(sortedSequences)]
        pub unsafe fn sortedSequences(&self) -> bool;

        #[deprecated]
        #[method_id(@__retain_semantics Other batchSizePerSequenceStep)]
        pub unsafe fn batchSizePerSequenceStep(&self) -> Option<Retained<NSArray<NSNumber>>>;

        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[deprecated]
        #[method(maxTensorDimensions)]
        pub unsafe fn maxTensorDimensions() -> NSUInteger;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptorWithShape:dataType:)]
        pub unsafe fn descriptorWithShape_dataType(
            shape: &NSArray<NSNumber>,
            data_type: MLCDataType,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptorWithShape:sequenceLengths:sortedSequences:dataType:)]
        pub unsafe fn descriptorWithShape_sequenceLengths_sortedSequences_dataType(
            shape: &NSArray<NSNumber>,
            sequence_lengths: &NSArray<NSNumber>,
            sorted_sequences: bool,
            data_type: MLCDataType,
        ) -> Option<Retained<Self>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other descriptorWithWidth:height:featureChannelCount:batchSize:)]
        pub unsafe fn descriptorWithWidth_height_featureChannelCount_batchSize(
            width: NSUInteger,
            height: NSUInteger,
            feature_channels: NSUInteger,
            batch_size: NSUInteger,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptorWithWidth:height:featureChannelCount:batchSize:dataType:)]
        pub unsafe fn descriptorWithWidth_height_featureChannelCount_batchSize_dataType(
            width: NSUInteger,
            height: NSUInteger,
            feature_channel_count: NSUInteger,
            batch_size: NSUInteger,
            data_type: MLCDataType,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other convolutionWeightsDescriptorWithWidth:height:inputFeatureChannelCount:outputFeatureChannelCount:dataType:)]
        pub unsafe fn convolutionWeightsDescriptorWithWidth_height_inputFeatureChannelCount_outputFeatureChannelCount_dataType(
            width: NSUInteger,
            height: NSUInteger,
            input_feature_channel_count: NSUInteger,
            output_feature_channel_count: NSUInteger,
            data_type: MLCDataType,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other convolutionWeightsDescriptorWithInputFeatureChannelCount:outputFeatureChannelCount:dataType:)]
        pub unsafe fn convolutionWeightsDescriptorWithInputFeatureChannelCount_outputFeatureChannelCount_dataType(
            input_feature_channel_count: NSUInteger,
            output_feature_channel_count: NSUInteger,
            data_type: MLCDataType,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other convolutionBiasesDescriptorWithFeatureChannelCount:dataType:)]
        pub unsafe fn convolutionBiasesDescriptorWithFeatureChannelCount_dataType(
            feature_channel_count: NSUInteger,
            data_type: MLCDataType,
        ) -> Option<Retained<Self>>;
    }
);

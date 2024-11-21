//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct MLCConvolutionDescriptor;
);

unsafe impl NSCopying for MLCConvolutionDescriptor {}

unsafe impl CopyingHelper for MLCConvolutionDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MLCConvolutionDescriptor {}

extern_methods!(
    unsafe impl MLCConvolutionDescriptor {
        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method(convolutionType)]
        pub unsafe fn convolutionType(&self) -> MLCConvolutionType;

        #[deprecated]
        #[method(kernelWidth)]
        pub unsafe fn kernelWidth(&self) -> NSUInteger;

        #[deprecated]
        #[method(kernelHeight)]
        pub unsafe fn kernelHeight(&self) -> NSUInteger;

        #[deprecated]
        #[method(inputFeatureChannelCount)]
        pub unsafe fn inputFeatureChannelCount(&self) -> NSUInteger;

        #[deprecated]
        #[method(outputFeatureChannelCount)]
        pub unsafe fn outputFeatureChannelCount(&self) -> NSUInteger;

        #[deprecated]
        #[method(strideInX)]
        pub unsafe fn strideInX(&self) -> NSUInteger;

        #[deprecated]
        #[method(strideInY)]
        pub unsafe fn strideInY(&self) -> NSUInteger;

        #[deprecated]
        #[method(dilationRateInX)]
        pub unsafe fn dilationRateInX(&self) -> NSUInteger;

        #[deprecated]
        #[method(dilationRateInY)]
        pub unsafe fn dilationRateInY(&self) -> NSUInteger;

        #[deprecated]
        #[method(groupCount)]
        pub unsafe fn groupCount(&self) -> NSUInteger;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method(paddingPolicy)]
        pub unsafe fn paddingPolicy(&self) -> MLCPaddingPolicy;

        #[deprecated]
        #[method(paddingSizeInX)]
        pub unsafe fn paddingSizeInX(&self) -> NSUInteger;

        #[deprecated]
        #[method(paddingSizeInY)]
        pub unsafe fn paddingSizeInY(&self) -> NSUInteger;

        #[deprecated]
        #[method(isConvolutionTranspose)]
        pub unsafe fn isConvolutionTranspose(&self) -> bool;

        #[deprecated]
        #[method(usesDepthwiseConvolution)]
        pub unsafe fn usesDepthwiseConvolution(&self) -> bool;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptorWithType:kernelSizes:inputFeatureChannelCount:outputFeatureChannelCount:groupCount:strides:dilationRates:paddingPolicy:paddingSizes:)]
        pub unsafe fn descriptorWithType_kernelSizes_inputFeatureChannelCount_outputFeatureChannelCount_groupCount_strides_dilationRates_paddingPolicy_paddingSizes(
            convolution_type: MLCConvolutionType,
            kernel_sizes: &NSArray<NSNumber>,
            input_feature_channel_count: NSUInteger,
            output_feature_channel_count: NSUInteger,
            group_count: NSUInteger,
            strides: &NSArray<NSNumber>,
            dilation_rates: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
        ) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other descriptorWithKernelWidth:kernelHeight:inputFeatureChannelCount:outputFeatureChannelCount:)]
        pub unsafe fn descriptorWithKernelWidth_kernelHeight_inputFeatureChannelCount_outputFeatureChannelCount(
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
            input_feature_channel_count: NSUInteger,
            output_feature_channel_count: NSUInteger,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptorWithKernelSizes:inputFeatureChannelCount:outputFeatureChannelCount:strides:paddingPolicy:paddingSizes:)]
        pub unsafe fn descriptorWithKernelSizes_inputFeatureChannelCount_outputFeatureChannelCount_strides_paddingPolicy_paddingSizes(
            kernel_sizes: &NSArray<NSNumber>,
            input_feature_channel_count: NSUInteger,
            output_feature_channel_count: NSUInteger,
            strides: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptorWithKernelSizes:inputFeatureChannelCount:outputFeatureChannelCount:groupCount:strides:dilationRates:paddingPolicy:paddingSizes:)]
        pub unsafe fn descriptorWithKernelSizes_inputFeatureChannelCount_outputFeatureChannelCount_groupCount_strides_dilationRates_paddingPolicy_paddingSizes(
            kernel_sizes: &NSArray<NSNumber>,
            input_feature_channel_count: NSUInteger,
            output_feature_channel_count: NSUInteger,
            group_count: NSUInteger,
            strides: &NSArray<NSNumber>,
            dilation_rates: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
        ) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other convolutionTransposeDescriptorWithKernelWidth:kernelHeight:inputFeatureChannelCount:outputFeatureChannelCount:)]
        pub unsafe fn convolutionTransposeDescriptorWithKernelWidth_kernelHeight_inputFeatureChannelCount_outputFeatureChannelCount(
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
            input_feature_channel_count: NSUInteger,
            output_feature_channel_count: NSUInteger,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other convolutionTransposeDescriptorWithKernelSizes:inputFeatureChannelCount:outputFeatureChannelCount:strides:paddingPolicy:paddingSizes:)]
        pub unsafe fn convolutionTransposeDescriptorWithKernelSizes_inputFeatureChannelCount_outputFeatureChannelCount_strides_paddingPolicy_paddingSizes(
            kernel_sizes: &NSArray<NSNumber>,
            input_feature_channel_count: NSUInteger,
            output_feature_channel_count: NSUInteger,
            strides: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other convolutionTransposeDescriptorWithKernelSizes:inputFeatureChannelCount:outputFeatureChannelCount:groupCount:strides:dilationRates:paddingPolicy:paddingSizes:)]
        pub unsafe fn convolutionTransposeDescriptorWithKernelSizes_inputFeatureChannelCount_outputFeatureChannelCount_groupCount_strides_dilationRates_paddingPolicy_paddingSizes(
            kernel_sizes: &NSArray<NSNumber>,
            input_feature_channel_count: NSUInteger,
            output_feature_channel_count: NSUInteger,
            group_count: NSUInteger,
            strides: &NSArray<NSNumber>,
            dilation_rates: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
        ) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other depthwiseConvolutionDescriptorWithKernelWidth:kernelHeight:inputFeatureChannelCount:channelMultiplier:)]
        pub unsafe fn depthwiseConvolutionDescriptorWithKernelWidth_kernelHeight_inputFeatureChannelCount_channelMultiplier(
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
            input_feature_channel_count: NSUInteger,
            channel_multiplier: NSUInteger,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other depthwiseConvolutionDescriptorWithKernelSizes:inputFeatureChannelCount:channelMultiplier:strides:paddingPolicy:paddingSizes:)]
        pub unsafe fn depthwiseConvolutionDescriptorWithKernelSizes_inputFeatureChannelCount_channelMultiplier_strides_paddingPolicy_paddingSizes(
            kernel_sizes: &NSArray<NSNumber>,
            input_feature_channel_count: NSUInteger,
            channel_multiplier: NSUInteger,
            strides: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other depthwiseConvolutionDescriptorWithKernelSizes:inputFeatureChannelCount:channelMultiplier:strides:dilationRates:paddingPolicy:paddingSizes:)]
        pub unsafe fn depthwiseConvolutionDescriptorWithKernelSizes_inputFeatureChannelCount_channelMultiplier_strides_dilationRates_paddingPolicy_paddingSizes(
            kernel_sizes: &NSArray<NSNumber>,
            input_feature_channel_count: NSUInteger,
            channel_multiplier: NSUInteger,
            strides: &NSArray<NSNumber>,
            dilation_rates: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MLCConvolutionDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

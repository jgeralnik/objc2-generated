//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct MLCPoolingDescriptor;

    unsafe impl ClassType for MLCPoolingDescriptor {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for MLCPoolingDescriptor {}

unsafe impl CopyingHelper for MLCPoolingDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MLCPoolingDescriptor {}

extern_methods!(
    unsafe impl MLCPoolingDescriptor {
        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method(poolingType)]
        pub unsafe fn poolingType(&self) -> MLCPoolingType;

        #[deprecated]
        #[method(kernelWidth)]
        pub unsafe fn kernelWidth(&self) -> NSUInteger;

        #[deprecated]
        #[method(kernelHeight)]
        pub unsafe fn kernelHeight(&self) -> NSUInteger;

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
        #[method(countIncludesPadding)]
        pub unsafe fn countIncludesPadding(&self) -> bool;

        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other poolingDescriptorWithType:kernelSize:stride:)]
        pub unsafe fn poolingDescriptorWithType_kernelSize_stride(
            pooling_type: MLCPoolingType,
            kernel_size: NSUInteger,
            stride: NSUInteger,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other maxPoolingDescriptorWithKernelSizes:strides:paddingPolicy:paddingSizes:)]
        pub unsafe fn maxPoolingDescriptorWithKernelSizes_strides_paddingPolicy_paddingSizes(
            kernel_sizes: &NSArray<NSNumber>,
            strides: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other maxPoolingDescriptorWithKernelSizes:strides:dilationRates:paddingPolicy:paddingSizes:)]
        pub unsafe fn maxPoolingDescriptorWithKernelSizes_strides_dilationRates_paddingPolicy_paddingSizes(
            kernel_sizes: &NSArray<NSNumber>,
            strides: &NSArray<NSNumber>,
            dilation_rates: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other averagePoolingDescriptorWithKernelSizes:strides:paddingPolicy:paddingSizes:countIncludesPadding:)]
        pub unsafe fn averagePoolingDescriptorWithKernelSizes_strides_paddingPolicy_paddingSizes_countIncludesPadding(
            kernel_sizes: &NSArray<NSNumber>,
            strides: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
            count_includes_padding: bool,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other averagePoolingDescriptorWithKernelSizes:strides:dilationRates:paddingPolicy:paddingSizes:countIncludesPadding:)]
        pub unsafe fn averagePoolingDescriptorWithKernelSizes_strides_dilationRates_paddingPolicy_paddingSizes_countIncludesPadding(
            kernel_sizes: &NSArray<NSNumber>,
            strides: &NSArray<NSNumber>,
            dilation_rates: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
            count_includes_padding: bool,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other l2NormPoolingDescriptorWithKernelSizes:strides:paddingPolicy:paddingSizes:)]
        pub unsafe fn l2NormPoolingDescriptorWithKernelSizes_strides_paddingPolicy_paddingSizes(
            kernel_sizes: &NSArray<NSNumber>,
            strides: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other l2NormPoolingDescriptorWithKernelSizes:strides:dilationRates:paddingPolicy:paddingSizes:)]
        pub unsafe fn l2NormPoolingDescriptorWithKernelSizes_strides_dilationRates_paddingPolicy_paddingSizes(
            kernel_sizes: &NSArray<NSNumber>,
            strides: &NSArray<NSNumber>,
            dilation_rates: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
        ) -> Retained<Self>;
    }
);

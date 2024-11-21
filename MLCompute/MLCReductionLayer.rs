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
    pub struct MLCReductionLayer;
);

#[cfg(feature = "MLCLayer")]
unsafe impl NSObjectProtocol for MLCReductionLayer {}

extern_methods!(
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCReductionLayer {
        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method(reductionType)]
        pub unsafe fn reductionType(&self) -> MLCReductionType;

        #[deprecated]
        #[method(dimension)]
        pub unsafe fn dimension(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other dimensions)]
        pub unsafe fn dimensions(&self) -> Retained<NSArray<NSNumber>>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithReductionType:dimension:)]
        pub unsafe fn layerWithReductionType_dimension(
            reduction_type: MLCReductionType,
            dimension: NSUInteger,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MLCTypes")]
        #[method_id(@__retain_semantics Other layerWithReductionType:dimensions:)]
        pub unsafe fn layerWithReductionType_dimensions(
            reduction_type: MLCReductionType,
            dimensions: &NSArray<NSNumber>,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCReductionLayer {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

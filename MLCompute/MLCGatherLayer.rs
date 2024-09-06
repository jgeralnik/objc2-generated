//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCLayer")]
    #[deprecated]
    pub struct MLCGatherLayer;

    #[cfg(feature = "MLCLayer")]
    unsafe impl ClassType for MLCGatherLayer {
        #[inherits(NSObject)]
        type Super = MLCLayer;
    }
);

#[cfg(feature = "MLCLayer")]
unsafe impl NSObjectProtocol for MLCGatherLayer {}

extern_methods!(
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCGatherLayer {
        #[deprecated]
        #[method(dimension)]
        pub unsafe fn dimension(&self) -> NSUInteger;

        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithDimension:)]
        pub unsafe fn layerWithDimension(dimension: NSUInteger) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCGatherLayer {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[unsafe(super(MLCLayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCLayer")]
    #[deprecated]
    pub struct MLCMatMulLayer;
);

#[cfg(feature = "MLCLayer")]
unsafe impl NSObjectProtocol for MLCMatMulLayer {}

extern_methods!(
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCMatMulLayer {
        #[cfg(feature = "MLCMatMulDescriptor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptor)]
        pub unsafe fn descriptor(&self) -> Retained<MLCMatMulDescriptor>;

        #[cfg(feature = "MLCMatMulDescriptor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithDescriptor:)]
        pub unsafe fn layerWithDescriptor(
            descriptor: &MLCMatMulDescriptor,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCMatMulLayer {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

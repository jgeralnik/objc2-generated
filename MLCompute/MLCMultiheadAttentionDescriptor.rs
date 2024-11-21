//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct MLCMultiheadAttentionDescriptor;
);

unsafe impl NSCopying for MLCMultiheadAttentionDescriptor {}

unsafe impl CopyingHelper for MLCMultiheadAttentionDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MLCMultiheadAttentionDescriptor {}

extern_methods!(
    unsafe impl MLCMultiheadAttentionDescriptor {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[deprecated]
        #[method(modelDimension)]
        pub unsafe fn modelDimension(&self) -> NSUInteger;

        #[deprecated]
        #[method(keyDimension)]
        pub unsafe fn keyDimension(&self) -> NSUInteger;

        #[deprecated]
        #[method(valueDimension)]
        pub unsafe fn valueDimension(&self) -> NSUInteger;

        #[deprecated]
        #[method(headCount)]
        pub unsafe fn headCount(&self) -> NSUInteger;

        #[deprecated]
        #[method(dropout)]
        pub unsafe fn dropout(&self) -> c_float;

        #[deprecated]
        #[method(hasBiases)]
        pub unsafe fn hasBiases(&self) -> bool;

        #[deprecated]
        #[method(hasAttentionBiases)]
        pub unsafe fn hasAttentionBiases(&self) -> bool;

        #[deprecated]
        #[method(addsZeroAttention)]
        pub unsafe fn addsZeroAttention(&self) -> bool;

        #[deprecated]
        #[method_id(@__retain_semantics Other descriptorWithModelDimension:keyDimension:valueDimension:headCount:dropout:hasBiases:hasAttentionBiases:addsZeroAttention:)]
        pub unsafe fn descriptorWithModelDimension_keyDimension_valueDimension_headCount_dropout_hasBiases_hasAttentionBiases_addsZeroAttention(
            model_dimension: NSUInteger,
            key_dimension: NSUInteger,
            value_dimension: NSUInteger,
            head_count: NSUInteger,
            dropout: c_float,
            has_biases: bool,
            has_attention_biases: bool,
            adds_zero_attention: bool,
        ) -> Option<Retained<Self>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other descriptorWithModelDimension:headCount:)]
        pub unsafe fn descriptorWithModelDimension_headCount(
            model_dimension: NSUInteger,
            head_count: NSUInteger,
        ) -> Retained<Self>;
    }
);

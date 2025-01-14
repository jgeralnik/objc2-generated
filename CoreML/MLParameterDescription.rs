//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreml/mlparameterdescription?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLParameterDescription;
);

unsafe impl NSCoding for MLParameterDescription {}

unsafe impl NSObjectProtocol for MLParameterDescription {}

unsafe impl NSSecureCoding for MLParameterDescription {}

extern_methods!(
    unsafe impl MLParameterDescription {
        #[cfg(all(feature = "MLKey", feature = "MLParameterKey"))]
        #[method_id(@__retain_semantics Other key)]
        pub unsafe fn key(&self) -> Retained<MLParameterKey>;

        #[method_id(@__retain_semantics Other defaultValue)]
        pub unsafe fn defaultValue(&self) -> Retained<AnyObject>;

        #[cfg(feature = "MLNumericConstraint")]
        #[method_id(@__retain_semantics Other numericConstraint)]
        pub unsafe fn numericConstraint(&self) -> Option<Retained<MLNumericConstraint>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MLParameterDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreml/mlmetrickey?language=objc)
    #[unsafe(super(MLKey, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLKey")]
    pub struct MLMetricKey;
);

#[cfg(feature = "MLKey")]
unsafe impl NSCoding for MLMetricKey {}

#[cfg(feature = "MLKey")]
unsafe impl NSCopying for MLMetricKey {}

#[cfg(feature = "MLKey")]
unsafe impl CopyingHelper for MLMetricKey {
    type Result = Self;
}

#[cfg(feature = "MLKey")]
unsafe impl NSObjectProtocol for MLMetricKey {}

#[cfg(feature = "MLKey")]
unsafe impl NSSecureCoding for MLMetricKey {}

extern_methods!(
    #[cfg(feature = "MLKey")]
    unsafe impl MLMetricKey {
        #[method_id(@__retain_semantics Other lossValue)]
        pub unsafe fn lossValue() -> Retained<MLMetricKey>;

        #[method_id(@__retain_semantics Other epochIndex)]
        pub unsafe fn epochIndex() -> Retained<MLMetricKey>;

        #[method_id(@__retain_semantics Other miniBatchIndex)]
        pub unsafe fn miniBatchIndex() -> Retained<MLMetricKey>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

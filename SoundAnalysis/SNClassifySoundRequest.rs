//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-ml")]
use objc2_core_ml::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SNClassifySoundRequest;
);

unsafe impl NSObjectProtocol for SNClassifySoundRequest {}

#[cfg(feature = "SNRequest")]
unsafe impl SNRequest for SNClassifySoundRequest {}

extern_methods!(
    unsafe impl SNClassifySoundRequest {
        #[method(overlapFactor)]
        pub unsafe fn overlapFactor(&self) -> c_double;

        #[method(setOverlapFactor:)]
        pub unsafe fn setOverlapFactor(&self, overlap_factor: c_double);

        #[cfg(feature = "SNTimeDurationConstraint")]
        #[method_id(@__retain_semantics Other windowDurationConstraint)]
        pub unsafe fn windowDurationConstraint(&self) -> Retained<SNTimeDurationConstraint>;

        #[method_id(@__retain_semantics Other knownClassifications)]
        pub unsafe fn knownClassifications(&self) -> Retained<NSArray<NSString>>;

        #[cfg(feature = "objc2-core-ml")]
        #[method_id(@__retain_semantics Init initWithMLModel:error:_)]
        pub unsafe fn initWithMLModel_error(
            this: Allocated<Self>,
            ml_model: &MLModel,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "SNTypes")]
        #[method_id(@__retain_semantics Init initWithClassifierIdentifier:error:_)]
        pub unsafe fn initWithClassifierIdentifier_error(
            this: Allocated<Self>,
            classifier_identifier: &SNClassifierIdentifier,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

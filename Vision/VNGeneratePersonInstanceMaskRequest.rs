//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VNRequest")]
    pub struct VNGeneratePersonInstanceMaskRequest;

    #[cfg(feature = "VNRequest")]
    unsafe impl ClassType for VNGeneratePersonInstanceMaskRequest {
        #[inherits(VNRequest, NSObject)]
        type Super = VNImageBasedRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "VNRequest")]
unsafe impl NSCopying for VNGeneratePersonInstanceMaskRequest {}

#[cfg(feature = "VNRequest")]
unsafe impl CopyingHelper for VNGeneratePersonInstanceMaskRequest {
    type Result = Self;
}

#[cfg(feature = "VNRequest")]
unsafe impl NSObjectProtocol for VNGeneratePersonInstanceMaskRequest {}

extern_methods!(
    #[cfg(feature = "VNRequest")]
    unsafe impl VNGeneratePersonInstanceMaskRequest {
        #[cfg(feature = "VNObservation")]
        #[method_id(@__retain_semantics Other results)]
        pub unsafe fn results(&self) -> Option<Retained<NSArray<VNInstanceMaskObservation>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VNRequest`
    #[cfg(feature = "VNRequest")]
    unsafe impl VNGeneratePersonInstanceMaskRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithCompletionHandler:)]
        pub unsafe fn initWithCompletionHandler(
            this: Allocated<Self>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "VNRequest")]
    unsafe impl VNGeneratePersonInstanceMaskRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

pub static VNGeneratePersonInstanceMaskRequestRevision1: NSUInteger = 1;

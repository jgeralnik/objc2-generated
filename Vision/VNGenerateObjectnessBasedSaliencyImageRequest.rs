//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vngenerateobjectnessbasedsaliencyimagerequest?language=objc)
    #[unsafe(super(VNImageBasedRequest, VNRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VNRequest")]
    pub struct VNGenerateObjectnessBasedSaliencyImageRequest;
);

#[cfg(feature = "VNRequest")]
unsafe impl NSCopying for VNGenerateObjectnessBasedSaliencyImageRequest {}

#[cfg(feature = "VNRequest")]
unsafe impl CopyingHelper for VNGenerateObjectnessBasedSaliencyImageRequest {
    type Result = Self;
}

#[cfg(feature = "VNRequest")]
unsafe impl NSObjectProtocol for VNGenerateObjectnessBasedSaliencyImageRequest {}

extern_methods!(
    #[cfg(feature = "VNRequest")]
    unsafe impl VNGenerateObjectnessBasedSaliencyImageRequest {
        #[cfg(feature = "VNObservation")]
        #[method_id(@__retain_semantics Other results)]
        pub unsafe fn results(&self) -> Option<Retained<NSArray<VNSaliencyImageObservation>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VNRequest`
    #[cfg(feature = "VNRequest")]
    unsafe impl VNGenerateObjectnessBasedSaliencyImageRequest {
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
    unsafe impl VNGenerateObjectnessBasedSaliencyImageRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vngenerateobjectnessbasedsaliencyimagerequestrevision1?language=objc)
pub static VNGenerateObjectnessBasedSaliencyImageRequestRevision1: NSUInteger = 1;

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vngenerateobjectnessbasedsaliencyimagerequestrevision2?language=objc)
pub static VNGenerateObjectnessBasedSaliencyImageRequestRevision2: NSUInteger = 2;

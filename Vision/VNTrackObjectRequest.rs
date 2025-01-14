//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vntrackobjectrequest?language=objc)
    #[unsafe(super(VNTrackingRequest, VNImageBasedRequest, VNRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "VNRequest", feature = "VNTrackingRequest"))]
    pub struct VNTrackObjectRequest;
);

#[cfg(all(feature = "VNRequest", feature = "VNTrackingRequest"))]
unsafe impl NSCopying for VNTrackObjectRequest {}

#[cfg(all(feature = "VNRequest", feature = "VNTrackingRequest"))]
unsafe impl CopyingHelper for VNTrackObjectRequest {
    type Result = Self;
}

#[cfg(all(feature = "VNRequest", feature = "VNTrackingRequest"))]
unsafe impl NSObjectProtocol for VNTrackObjectRequest {}

extern_methods!(
    #[cfg(all(feature = "VNRequest", feature = "VNTrackingRequest"))]
    unsafe impl VNTrackObjectRequest {
        #[cfg(feature = "VNObservation")]
        #[method_id(@__retain_semantics Init initWithDetectedObjectObservation:)]
        pub unsafe fn initWithDetectedObjectObservation(
            this: Allocated<Self>,
            observation: &VNDetectedObjectObservation,
        ) -> Retained<Self>;

        #[cfg(all(feature = "VNObservation", feature = "block2"))]
        #[method_id(@__retain_semantics Init initWithDetectedObjectObservation:completionHandler:)]
        pub unsafe fn initWithDetectedObjectObservation_completionHandler(
            this: Allocated<Self>,
            observation: &VNDetectedObjectObservation,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;

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
    #[cfg(all(feature = "VNRequest", feature = "VNTrackingRequest"))]
    unsafe impl VNTrackObjectRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vntrackobjectrequestrevision1?language=objc)
pub static VNTrackObjectRequestRevision1: NSUInteger = 1;

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vntrackobjectrequestrevision2?language=objc)
pub static VNTrackObjectRequestRevision2: NSUInteger = 2;

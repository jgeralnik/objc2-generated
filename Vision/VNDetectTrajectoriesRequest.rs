//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(VNStatefulRequest, VNImageBasedRequest, VNRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
    pub struct VNDetectTrajectoriesRequest;
);

#[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
unsafe impl NSCopying for VNDetectTrajectoriesRequest {}

#[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
unsafe impl CopyingHelper for VNDetectTrajectoriesRequest {
    type Result = Self;
}

#[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
unsafe impl NSObjectProtocol for VNDetectTrajectoriesRequest {}

extern_methods!(
    #[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
    unsafe impl VNDetectTrajectoriesRequest {
        #[method(trajectoryLength)]
        pub unsafe fn trajectoryLength(&self) -> NSInteger;

        #[method(objectMinimumNormalizedRadius)]
        pub unsafe fn objectMinimumNormalizedRadius(&self) -> c_float;

        #[method(setObjectMinimumNormalizedRadius:)]
        pub unsafe fn setObjectMinimumNormalizedRadius(
            &self,
            object_minimum_normalized_radius: c_float,
        );

        #[deprecated]
        #[method(minimumObjectSize)]
        pub unsafe fn minimumObjectSize(&self) -> c_float;

        #[deprecated]
        #[method(setMinimumObjectSize:)]
        pub unsafe fn setMinimumObjectSize(&self, minimum_object_size: c_float);

        #[method(objectMaximumNormalizedRadius)]
        pub unsafe fn objectMaximumNormalizedRadius(&self) -> c_float;

        #[method(setObjectMaximumNormalizedRadius:)]
        pub unsafe fn setObjectMaximumNormalizedRadius(
            &self,
            object_maximum_normalized_radius: c_float,
        );

        #[deprecated]
        #[method(maximumObjectSize)]
        pub unsafe fn maximumObjectSize(&self) -> c_float;

        #[deprecated]
        #[method(setMaximumObjectSize:)]
        pub unsafe fn setMaximumObjectSize(&self, maximum_object_size: c_float);

        #[cfg(feature = "VNObservation")]
        #[method_id(@__retain_semantics Other results)]
        pub unsafe fn results(&self) -> Option<Retained<NSArray<VNTrajectoryObservation>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VNStatefulRequest`
    #[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
    unsafe impl VNDetectTrajectoriesRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

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

pub static VNDetectTrajectoriesRequestRevision1: NSUInteger = 1;

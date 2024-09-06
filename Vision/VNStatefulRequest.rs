//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VNRequest")]
    pub struct VNStatefulRequest;

    #[cfg(feature = "VNRequest")]
    unsafe impl ClassType for VNStatefulRequest {
        #[inherits(VNRequest, NSObject)]
        type Super = VNImageBasedRequest;
    }
);

#[cfg(feature = "VNRequest")]
unsafe impl NSCopying for VNStatefulRequest {}

#[cfg(feature = "VNRequest")]
unsafe impl CopyingHelper for VNStatefulRequest {
    type Result = Self;
}

#[cfg(feature = "VNRequest")]
unsafe impl NSObjectProtocol for VNStatefulRequest {}

extern_methods!(
    #[cfg(feature = "VNRequest")]
    unsafe impl VNStatefulRequest {
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

        #[method(minimumLatencyFrameCount)]
        pub unsafe fn minimumLatencyFrameCount(&self) -> NSInteger;
    }
);

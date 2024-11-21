//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-image")]
use objc2_core_image::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(VNImageBasedRequest, VNRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VNRequest")]
    pub struct VNTargetedImageRequest;
);

#[cfg(feature = "VNRequest")]
unsafe impl NSCopying for VNTargetedImageRequest {}

#[cfg(feature = "VNRequest")]
unsafe impl CopyingHelper for VNTargetedImageRequest {
    type Result = Self;
}

#[cfg(feature = "VNRequest")]
unsafe impl NSObjectProtocol for VNTargetedImageRequest {}

extern_methods!(
    #[cfg(feature = "VNRequest")]
    unsafe impl VNTargetedImageRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithCompletionHandler:)]
        pub unsafe fn initWithCompletionHandler(
            this: Allocated<Self>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;

        #[cfg(all(feature = "VNRequestHandler", feature = "objc2-core-image"))]
        #[method_id(@__retain_semantics Init initWithTargetedCIImage:options:)]
        pub unsafe fn initWithTargetedCIImage_options(
            this: Allocated<Self>,
            ci_image: &CIImage,
            options: &NSDictionary<VNImageOption, AnyObject>,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "VNRequestHandler",
            feature = "block2",
            feature = "objc2-core-image"
        ))]
        #[method_id(@__retain_semantics Init initWithTargetedCIImage:options:completionHandler:)]
        pub unsafe fn initWithTargetedCIImage_options_completionHandler(
            this: Allocated<Self>,
            ci_image: &CIImage,
            options: &NSDictionary<VNImageOption, AnyObject>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;

        #[cfg(feature = "VNRequestHandler")]
        #[method_id(@__retain_semantics Init initWithTargetedImageURL:options:)]
        pub unsafe fn initWithTargetedImageURL_options(
            this: Allocated<Self>,
            image_url: &NSURL,
            options: &NSDictionary<VNImageOption, AnyObject>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "VNRequestHandler", feature = "block2"))]
        #[method_id(@__retain_semantics Init initWithTargetedImageURL:options:completionHandler:)]
        pub unsafe fn initWithTargetedImageURL_options_completionHandler(
            this: Allocated<Self>,
            image_url: &NSURL,
            options: &NSDictionary<VNImageOption, AnyObject>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;

        #[cfg(feature = "VNRequestHandler")]
        #[method_id(@__retain_semantics Init initWithTargetedImageData:options:)]
        pub unsafe fn initWithTargetedImageData_options(
            this: Allocated<Self>,
            image_data: &NSData,
            options: &NSDictionary<VNImageOption, AnyObject>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "VNRequestHandler", feature = "block2"))]
        #[method_id(@__retain_semantics Init initWithTargetedImageData:options:completionHandler:)]
        pub unsafe fn initWithTargetedImageData_options_completionHandler(
            this: Allocated<Self>,
            image_data: &NSData,
            options: &NSDictionary<VNImageOption, AnyObject>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "VNRequest")]
    unsafe impl VNTargetedImageRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

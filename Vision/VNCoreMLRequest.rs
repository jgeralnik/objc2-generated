//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-ml")]
use objc2_core_ml::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VNCoreMLModel;

    unsafe impl ClassType for VNCoreMLModel {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for VNCoreMLModel {}

extern_methods!(
    unsafe impl VNCoreMLModel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "objc2-core-ml")]
        #[method_id(@__retain_semantics Other modelForMLModel:error:_)]
        pub unsafe fn modelForMLModel_error(
            model: &MLModel,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method_id(@__retain_semantics Other inputImageFeatureName)]
        pub unsafe fn inputImageFeatureName(&self) -> Retained<NSString>;

        #[method(setInputImageFeatureName:)]
        pub unsafe fn setInputImageFeatureName(&self, input_image_feature_name: &NSString);

        #[cfg(feature = "objc2-core-ml")]
        #[method_id(@__retain_semantics Other featureProvider)]
        pub unsafe fn featureProvider(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MLFeatureProvider>>>;

        #[cfg(feature = "objc2-core-ml")]
        #[method(setFeatureProvider:)]
        pub unsafe fn setFeatureProvider(
            &self,
            feature_provider: Option<&ProtocolObject<dyn MLFeatureProvider>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl VNCoreMLModel {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VNRequest")]
    pub struct VNCoreMLRequest;

    #[cfg(feature = "VNRequest")]
    unsafe impl ClassType for VNCoreMLRequest {
        #[inherits(VNRequest, NSObject)]
        type Super = VNImageBasedRequest;
    }
);

#[cfg(feature = "VNRequest")]
unsafe impl NSCopying for VNCoreMLRequest {}

#[cfg(feature = "VNRequest")]
unsafe impl CopyingHelper for VNCoreMLRequest {
    type Result = Self;
}

#[cfg(feature = "VNRequest")]
unsafe impl NSObjectProtocol for VNCoreMLRequest {}

extern_methods!(
    #[cfg(feature = "VNRequest")]
    unsafe impl VNCoreMLRequest {
        #[method_id(@__retain_semantics Other model)]
        pub unsafe fn model(&self) -> Retained<VNCoreMLModel>;

        #[cfg(feature = "VNTypes")]
        #[method(imageCropAndScaleOption)]
        pub unsafe fn imageCropAndScaleOption(&self) -> VNImageCropAndScaleOption;

        #[cfg(feature = "VNTypes")]
        #[method(setImageCropAndScaleOption:)]
        pub unsafe fn setImageCropAndScaleOption(
            &self,
            image_crop_and_scale_option: VNImageCropAndScaleOption,
        );

        #[method_id(@__retain_semantics Init initWithModel:)]
        pub unsafe fn initWithModel(this: Allocated<Self>, model: &VNCoreMLModel)
            -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithModel:completionHandler:)]
        pub unsafe fn initWithModel_completionHandler(
            this: Allocated<Self>,
            model: &VNCoreMLModel,
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
    #[cfg(feature = "VNRequest")]
    unsafe impl VNCoreMLRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

pub static VNCoreMLRequestRevision1: NSUInteger = 1;

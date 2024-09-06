//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIFeature;

    unsafe impl ClassType for CIFeature {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for CIFeature {}

extern_methods!(
    unsafe impl CIFeature {
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Retained<NSString>;

        #[method(bounds)]
        pub unsafe fn bounds(&self) -> CGRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIFeature {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    pub static CIFeatureTypeFace: &'static NSString;
}

extern "C" {
    pub static CIFeatureTypeRectangle: &'static NSString;
}

extern "C" {
    pub static CIFeatureTypeQRCode: &'static NSString;
}

extern "C" {
    pub static CIFeatureTypeText: &'static NSString;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIFaceFeature;

    unsafe impl ClassType for CIFaceFeature {
        #[inherits(NSObject)]
        type Super = CIFeature;
    }
);

unsafe impl NSObjectProtocol for CIFaceFeature {}

extern_methods!(
    unsafe impl CIFaceFeature {
        #[method(bounds)]
        pub unsafe fn bounds(&self) -> CGRect;

        #[method(hasLeftEyePosition)]
        pub unsafe fn hasLeftEyePosition(&self) -> bool;

        #[method(leftEyePosition)]
        pub unsafe fn leftEyePosition(&self) -> CGPoint;

        #[method(hasRightEyePosition)]
        pub unsafe fn hasRightEyePosition(&self) -> bool;

        #[method(rightEyePosition)]
        pub unsafe fn rightEyePosition(&self) -> CGPoint;

        #[method(hasMouthPosition)]
        pub unsafe fn hasMouthPosition(&self) -> bool;

        #[method(mouthPosition)]
        pub unsafe fn mouthPosition(&self) -> CGPoint;

        #[method(hasTrackingID)]
        pub unsafe fn hasTrackingID(&self) -> bool;

        #[method(trackingID)]
        pub unsafe fn trackingID(&self) -> c_int;

        #[method(hasTrackingFrameCount)]
        pub unsafe fn hasTrackingFrameCount(&self) -> bool;

        #[method(trackingFrameCount)]
        pub unsafe fn trackingFrameCount(&self) -> c_int;

        #[method(hasFaceAngle)]
        pub unsafe fn hasFaceAngle(&self) -> bool;

        #[method(faceAngle)]
        pub unsafe fn faceAngle(&self) -> c_float;

        #[method(hasSmile)]
        pub unsafe fn hasSmile(&self) -> bool;

        #[method(leftEyeClosed)]
        pub unsafe fn leftEyeClosed(&self) -> bool;

        #[method(rightEyeClosed)]
        pub unsafe fn rightEyeClosed(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIFaceFeature {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIRectangleFeature;

    unsafe impl ClassType for CIRectangleFeature {
        #[inherits(NSObject)]
        type Super = CIFeature;
    }
);

unsafe impl NSObjectProtocol for CIRectangleFeature {}

extern_methods!(
    unsafe impl CIRectangleFeature {
        #[method(bounds)]
        pub unsafe fn bounds(&self) -> CGRect;

        #[method(topLeft)]
        pub unsafe fn topLeft(&self) -> CGPoint;

        #[method(topRight)]
        pub unsafe fn topRight(&self) -> CGPoint;

        #[method(bottomLeft)]
        pub unsafe fn bottomLeft(&self) -> CGPoint;

        #[method(bottomRight)]
        pub unsafe fn bottomRight(&self) -> CGPoint;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIRectangleFeature {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIQRCodeFeature;

    unsafe impl ClassType for CIQRCodeFeature {
        #[inherits(NSObject)]
        type Super = CIFeature;
    }
);

unsafe impl NSCoding for CIQRCodeFeature {}

unsafe impl NSCopying for CIQRCodeFeature {}

unsafe impl CopyingHelper for CIQRCodeFeature {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CIQRCodeFeature {}

unsafe impl NSSecureCoding for CIQRCodeFeature {}

extern_methods!(
    unsafe impl CIQRCodeFeature {
        #[method(bounds)]
        pub unsafe fn bounds(&self) -> CGRect;

        #[method(topLeft)]
        pub unsafe fn topLeft(&self) -> CGPoint;

        #[method(topRight)]
        pub unsafe fn topRight(&self) -> CGPoint;

        #[method(bottomLeft)]
        pub unsafe fn bottomLeft(&self) -> CGPoint;

        #[method(bottomRight)]
        pub unsafe fn bottomRight(&self) -> CGPoint;

        #[method_id(@__retain_semantics Other messageString)]
        pub unsafe fn messageString(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "CIBarcodeDescriptor")]
        #[method_id(@__retain_semantics Other symbolDescriptor)]
        pub unsafe fn symbolDescriptor(&self) -> Option<Retained<CIQRCodeDescriptor>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIQRCodeFeature {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CITextFeature;

    unsafe impl ClassType for CITextFeature {
        #[inherits(NSObject)]
        type Super = CIFeature;
    }
);

unsafe impl NSObjectProtocol for CITextFeature {}

extern_methods!(
    unsafe impl CITextFeature {
        #[method(bounds)]
        pub unsafe fn bounds(&self) -> CGRect;

        #[method(topLeft)]
        pub unsafe fn topLeft(&self) -> CGPoint;

        #[method(topRight)]
        pub unsafe fn topRight(&self) -> CGPoint;

        #[method(bottomLeft)]
        pub unsafe fn bottomLeft(&self) -> CGPoint;

        #[method(bottomRight)]
        pub unsafe fn bottomRight(&self) -> CGPoint;

        #[method_id(@__retain_semantics Other subFeatures)]
        pub unsafe fn subFeatures(&self) -> Option<Retained<NSArray>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CITextFeature {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

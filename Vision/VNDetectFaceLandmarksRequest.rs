//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vnrequestfacelandmarksconstellation?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VNRequestFaceLandmarksConstellation(pub NSUInteger);
impl VNRequestFaceLandmarksConstellation {
    #[doc(alias = "VNRequestFaceLandmarksConstellationNotDefined")]
    pub const NotDefined: Self = Self(0);
    pub const VNRequestFaceLandmarksConstellation65Points: Self = Self(1);
    pub const VNRequestFaceLandmarksConstellation76Points: Self = Self(2);
}

unsafe impl Encode for VNRequestFaceLandmarksConstellation {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for VNRequestFaceLandmarksConstellation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vndetectfacelandmarksrequest?language=objc)
    #[unsafe(super(VNImageBasedRequest, VNRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VNRequest")]
    pub struct VNDetectFaceLandmarksRequest;
);

#[cfg(feature = "VNRequest")]
unsafe impl NSCopying for VNDetectFaceLandmarksRequest {}

#[cfg(feature = "VNRequest")]
unsafe impl CopyingHelper for VNDetectFaceLandmarksRequest {
    type Result = Self;
}

#[cfg(feature = "VNRequest")]
unsafe impl NSObjectProtocol for VNDetectFaceLandmarksRequest {}

#[cfg(all(feature = "VNFaceObservationAccepting", feature = "VNRequest"))]
unsafe impl VNFaceObservationAccepting for VNDetectFaceLandmarksRequest {}

extern_methods!(
    #[cfg(feature = "VNRequest")]
    unsafe impl VNDetectFaceLandmarksRequest {
        #[method(revision:supportsConstellation:)]
        pub unsafe fn revision_supportsConstellation(
            request_revision: NSUInteger,
            constellation: VNRequestFaceLandmarksConstellation,
        ) -> bool;

        #[method(constellation)]
        pub unsafe fn constellation(&self) -> VNRequestFaceLandmarksConstellation;

        #[method(setConstellation:)]
        pub unsafe fn setConstellation(&self, constellation: VNRequestFaceLandmarksConstellation);

        #[cfg(feature = "VNObservation")]
        #[method_id(@__retain_semantics Other results)]
        pub unsafe fn results(&self) -> Option<Retained<NSArray<VNFaceObservation>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VNRequest`
    #[cfg(feature = "VNRequest")]
    unsafe impl VNDetectFaceLandmarksRequest {
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
    unsafe impl VNDetectFaceLandmarksRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vndetectfacelandmarksrequestrevision1?language=objc)
pub static VNDetectFaceLandmarksRequestRevision1: NSUInteger = 1;

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vndetectfacelandmarksrequestrevision2?language=objc)
pub static VNDetectFaceLandmarksRequestRevision2: NSUInteger = 2;

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vndetectfacelandmarksrequestrevision3?language=objc)
pub static VNDetectFaceLandmarksRequestRevision3: NSUInteger = 3;

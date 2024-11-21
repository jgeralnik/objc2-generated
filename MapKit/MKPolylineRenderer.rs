//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(MKOverlayPathRenderer, MKOverlayRenderer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MKOverlayPathRenderer", feature = "MKOverlayRenderer"))]
    pub struct MKPolylineRenderer;
);

#[cfg(all(feature = "MKOverlayPathRenderer", feature = "MKOverlayRenderer"))]
unsafe impl NSObjectProtocol for MKPolylineRenderer {}

extern_methods!(
    #[cfg(all(feature = "MKOverlayPathRenderer", feature = "MKOverlayRenderer"))]
    unsafe impl MKPolylineRenderer {
        #[cfg(all(feature = "MKMultiPoint", feature = "MKPolyline", feature = "MKShape"))]
        #[method_id(@__retain_semantics Init initWithPolyline:)]
        pub unsafe fn initWithPolyline(
            this: Allocated<Self>,
            polyline: &MKPolyline,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MKMultiPoint", feature = "MKPolyline", feature = "MKShape"))]
        #[method_id(@__retain_semantics Other polyline)]
        pub unsafe fn polyline(&self) -> Retained<MKPolyline>;

        #[method(strokeStart)]
        pub unsafe fn strokeStart(&self) -> CGFloat;

        #[method(setStrokeStart:)]
        pub unsafe fn setStrokeStart(&self, stroke_start: CGFloat);

        #[method(strokeEnd)]
        pub unsafe fn strokeEnd(&self) -> CGFloat;

        #[method(setStrokeEnd:)]
        pub unsafe fn setStrokeEnd(&self, stroke_end: CGFloat);
    }
);

extern_methods!(
    /// Methods declared on superclass `MKOverlayRenderer`
    #[cfg(all(feature = "MKOverlayPathRenderer", feature = "MKOverlayRenderer"))]
    unsafe impl MKPolylineRenderer {
        #[cfg(all(feature = "MKAnnotation", feature = "MKOverlay"))]
        #[method_id(@__retain_semantics Init initWithOverlay:)]
        pub unsafe fn initWithOverlay(
            this: Allocated<Self>,
            overlay: &ProtocolObject<dyn MKOverlay>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MKOverlayPathRenderer", feature = "MKOverlayRenderer"))]
    unsafe impl MKPolylineRenderer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

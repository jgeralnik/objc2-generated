//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKPolylineRenderer")]
    pub struct MKPolylineRenderer;

    #[cfg(feature = "MapKit_MKPolylineRenderer")]
    unsafe impl ClassType for MKPolylineRenderer {
        #[inherits(MKOverlayRenderer, NSObject)]
        type Super = MKOverlayPathRenderer;
    }
);

#[cfg(feature = "MapKit_MKPolylineRenderer")]
unsafe impl NSObjectProtocol for MKPolylineRenderer {}

extern_methods!(
    #[cfg(feature = "MapKit_MKPolylineRenderer")]
    unsafe impl MKPolylineRenderer {
        #[cfg(feature = "MapKit_MKPolyline")]
        #[method_id(@__retain_semantics Init initWithPolyline:)]
        pub unsafe fn initWithPolyline(
            this: Option<Allocated<Self>>,
            polyline: &MKPolyline,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "MapKit_MKPolyline")]
        #[method_id(@__retain_semantics Other polyline)]
        pub unsafe fn polyline(&self) -> Id<MKPolyline, Shared>;

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
    #[cfg(feature = "MapKit_MKPolylineRenderer")]
    unsafe impl MKPolylineRenderer {
        #[method_id(@__retain_semantics Init initWithOverlay:)]
        pub unsafe fn initWithOverlay(
            this: Option<Allocated<Self>>,
            overlay: &ProtocolObject<dyn MKOverlay>,
        ) -> Id<Self, Shared>;
    }
);
//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MKShape")]
    pub struct MKCircle;

    #[cfg(feature = "MKShape")]
    unsafe impl ClassType for MKCircle {
        #[inherits(NSObject)]
        type Super = MKShape;
    }
);

#[cfg(all(feature = "MKAnnotation", feature = "MKShape"))]
unsafe impl MKAnnotation for MKCircle {}

#[cfg(all(feature = "MKAnnotation", feature = "MKOverlay", feature = "MKShape"))]
unsafe impl MKOverlay for MKCircle {}

#[cfg(feature = "MKShape")]
unsafe impl NSObjectProtocol for MKCircle {}

extern_methods!(
    #[cfg(feature = "MKShape")]
    unsafe impl MKCircle {
        #[cfg(feature = "objc2-core-location")]
        #[method_id(@__retain_semantics Other circleWithCenterCoordinate:radius:)]
        pub unsafe fn circleWithCenterCoordinate_radius(
            coord: CLLocationCoordinate2D,
            radius: CLLocationDistance,
        ) -> Retained<Self>;

        #[cfg(feature = "MKGeometry")]
        #[method_id(@__retain_semantics Other circleWithMapRect:)]
        pub unsafe fn circleWithMapRect(map_rect: MKMapRect) -> Retained<Self>;

        #[cfg(feature = "objc2-core-location")]
        #[method(coordinate)]
        pub unsafe fn coordinate(&self) -> CLLocationCoordinate2D;

        #[cfg(feature = "objc2-core-location")]
        #[method(radius)]
        pub unsafe fn radius(&self) -> CLLocationDistance;

        #[cfg(feature = "MKGeometry")]
        #[method(boundingMapRect)]
        pub unsafe fn boundingMapRect(&self) -> MKMapRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MKShape")]
    unsafe impl MKCircle {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

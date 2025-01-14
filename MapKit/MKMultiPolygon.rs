//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkmultipolygon?language=objc)
    #[unsafe(super(MKShape, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MKShape")]
    pub struct MKMultiPolygon;
);

#[cfg(all(feature = "MKAnnotation", feature = "MKShape"))]
unsafe impl MKAnnotation for MKMultiPolygon {}

#[cfg(all(feature = "MKAnnotation", feature = "MKOverlay", feature = "MKShape"))]
unsafe impl MKOverlay for MKMultiPolygon {}

#[cfg(feature = "MKShape")]
unsafe impl NSObjectProtocol for MKMultiPolygon {}

extern_methods!(
    #[cfg(feature = "MKShape")]
    unsafe impl MKMultiPolygon {
        #[cfg(all(feature = "MKMultiPoint", feature = "MKPolygon"))]
        #[method_id(@__retain_semantics Init initWithPolygons:)]
        pub unsafe fn initWithPolygons(
            this: Allocated<Self>,
            polygons: &NSArray<MKPolygon>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MKMultiPoint", feature = "MKPolygon"))]
        #[method_id(@__retain_semantics Other polygons)]
        pub unsafe fn polygons(&self) -> Retained<NSArray<MKPolygon>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MKShape")]
    unsafe impl MKMultiPolygon {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

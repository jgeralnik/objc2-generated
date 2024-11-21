//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(MKShape, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MKShape")]
    pub struct MKMultiPolyline;
);

#[cfg(all(feature = "MKAnnotation", feature = "MKShape"))]
unsafe impl MKAnnotation for MKMultiPolyline {}

#[cfg(all(feature = "MKAnnotation", feature = "MKOverlay", feature = "MKShape"))]
unsafe impl MKOverlay for MKMultiPolyline {}

#[cfg(feature = "MKShape")]
unsafe impl NSObjectProtocol for MKMultiPolyline {}

extern_methods!(
    #[cfg(feature = "MKShape")]
    unsafe impl MKMultiPolyline {
        #[cfg(all(feature = "MKMultiPoint", feature = "MKPolyline"))]
        #[method_id(@__retain_semantics Init initWithPolylines:)]
        pub unsafe fn initWithPolylines(
            this: Allocated<Self>,
            polylines: &NSArray<MKPolyline>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MKMultiPoint", feature = "MKPolyline"))]
        #[method_id(@__retain_semantics Other polylines)]
        pub unsafe fn polylines(&self) -> Retained<NSArray<MKPolyline>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MKShape")]
    unsafe impl MKMultiPolyline {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

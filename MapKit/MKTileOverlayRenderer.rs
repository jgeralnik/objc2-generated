//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mktileoverlayrenderer?language=objc)
    #[unsafe(super(MKOverlayRenderer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MKOverlayRenderer")]
    pub struct MKTileOverlayRenderer;
);

#[cfg(feature = "MKOverlayRenderer")]
unsafe impl NSObjectProtocol for MKTileOverlayRenderer {}

extern_methods!(
    #[cfg(feature = "MKOverlayRenderer")]
    unsafe impl MKTileOverlayRenderer {
        #[cfg(feature = "MKTileOverlay")]
        #[method_id(@__retain_semantics Init initWithTileOverlay:)]
        pub unsafe fn initWithTileOverlay(
            this: Allocated<Self>,
            overlay: &MKTileOverlay,
        ) -> Retained<Self>;

        #[method(reloadData)]
        pub unsafe fn reloadData(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `MKOverlayRenderer`
    #[cfg(feature = "MKOverlayRenderer")]
    unsafe impl MKTileOverlayRenderer {
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
    #[cfg(feature = "MKOverlayRenderer")]
    unsafe impl MKTileOverlayRenderer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

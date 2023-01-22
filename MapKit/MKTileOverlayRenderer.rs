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
    #[cfg(feature = "MapKit_MKTileOverlayRenderer")]
    pub struct MKTileOverlayRenderer;

    #[cfg(feature = "MapKit_MKTileOverlayRenderer")]
    unsafe impl ClassType for MKTileOverlayRenderer {
        #[inherits(NSObject)]
        type Super = MKOverlayRenderer;
    }
);

#[cfg(feature = "MapKit_MKTileOverlayRenderer")]
unsafe impl NSObjectProtocol for MKTileOverlayRenderer {}

extern_methods!(
    #[cfg(feature = "MapKit_MKTileOverlayRenderer")]
    unsafe impl MKTileOverlayRenderer {
        #[cfg(feature = "MapKit_MKTileOverlay")]
        #[method_id(@__retain_semantics Init initWithTileOverlay:)]
        pub unsafe fn initWithTileOverlay(
            this: Option<Allocated<Self>>,
            overlay: &MKTileOverlay,
        ) -> Id<Self, Shared>;

        #[method(reloadData)]
        pub unsafe fn reloadData(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `MKOverlayRenderer`
    #[cfg(feature = "MapKit_MKTileOverlayRenderer")]
    unsafe impl MKTileOverlayRenderer {
        #[method_id(@__retain_semantics Init initWithOverlay:)]
        pub unsafe fn initWithOverlay(
            this: Option<Allocated<Self>>,
            overlay: &ProtocolObject<dyn MKOverlay>,
        ) -> Id<Self, Shared>;
    }
);
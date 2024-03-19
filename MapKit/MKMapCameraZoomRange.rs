//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern "C" {
    #[cfg(feature = "CoreLocation_CLLocation")]
    pub static MKMapCameraZoomDefault: CLLocationDistance;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKMapCameraZoomRange;

    unsafe impl ClassType for MKMapCameraZoomRange {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for MKMapCameraZoomRange {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for MKMapCameraZoomRange {}

unsafe impl NSObjectProtocol for MKMapCameraZoomRange {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for MKMapCameraZoomRange {}

extern_methods!(
    unsafe impl MKMapCameraZoomRange {
        #[cfg(feature = "CoreLocation_CLLocation")]
        #[method_id(@__retain_semantics Init initWithMinCenterCoordinateDistance:maxCenterCoordinateDistance:)]
        pub unsafe fn initWithMinCenterCoordinateDistance_maxCenterCoordinateDistance(
            this: Allocated<Self>,
            min_distance: CLLocationDistance,
            max_distance: CLLocationDistance,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "CoreLocation_CLLocation")]
        #[method_id(@__retain_semantics Init initWithMinCenterCoordinateDistance:)]
        pub unsafe fn initWithMinCenterCoordinateDistance(
            this: Allocated<Self>,
            min_distance: CLLocationDistance,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "CoreLocation_CLLocation")]
        #[method_id(@__retain_semantics Init initWithMaxCenterCoordinateDistance:)]
        pub unsafe fn initWithMaxCenterCoordinateDistance(
            this: Allocated<Self>,
            max_distance: CLLocationDistance,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "CoreLocation_CLLocation")]
        #[method(minCenterCoordinateDistance)]
        pub unsafe fn minCenterCoordinateDistance(&self) -> CLLocationDistance;

        #[cfg(feature = "CoreLocation_CLLocation")]
        #[method(maxCenterCoordinateDistance)]
        pub unsafe fn maxCenterCoordinateDistance(&self) -> CLLocationDistance;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKMapCameraZoomRange {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

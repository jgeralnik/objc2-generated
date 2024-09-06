//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKMapSnapshot;

    unsafe impl ClassType for MKMapSnapshot {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for MKMapSnapshot {}

extern_methods!(
    unsafe impl MKMapSnapshot {
        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Retained<NSImage>;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Other appearance)]
        pub unsafe fn appearance(&self) -> Retained<NSAppearance>;

        #[cfg(feature = "objc2-core-location")]
        #[method(pointForCoordinate:)]
        pub unsafe fn pointForCoordinate(&self, coordinate: CLLocationCoordinate2D) -> NSPoint;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKMapSnapshot {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

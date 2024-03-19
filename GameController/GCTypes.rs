//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_struct!(
    pub struct GCPoint2 {
        pub x: c_float,
        pub y: c_float,
    }
);

extern "C" {
    pub static GCPoint2Zero: GCPoint2;
}

// TODO: pub fn GCPoint2Make(x: c_float,y: c_float,) -> GCPoint2;

// TODO: pub fn GCPoint2Equal(point1: GCPoint2,point2: GCPoint2,) -> bool;

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub fn NSStringFromGCPoint2(point: GCPoint2) -> NonNull<NSString>;
}

extern_category!(
    /// Category "GCTypes" on [`NSValue`].
    #[doc(alias = "GCTypes")]
    pub unsafe trait NSValueGCTypes {
        #[cfg(feature = "GameController_GCTypes")]
        #[method_id(@__retain_semantics Other valueWithGCPoint2:)]
        unsafe fn valueWithGCPoint2(point: GCPoint2) -> Id<Self>;

        #[cfg(feature = "GameController_GCTypes")]
        #[method(GCPoint2Value)]
        unsafe fn GCPoint2Value(&self) -> GCPoint2;
    }

    #[cfg(feature = "Foundation_NSValue")]
    unsafe impl NSValueGCTypes for NSValue {}
);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VNGeometryUtils;
);

unsafe impl NSObjectProtocol for VNGeometryUtils {}

extern_methods!(
    unsafe impl VNGeometryUtils {
        #[cfg(feature = "VNGeometry")]
        #[method_id(@__retain_semantics Other boundingCircleForContour:error:_)]
        pub unsafe fn boundingCircleForContour_error(
            contour: &VNContour,
        ) -> Result<Retained<VNCircle>, Retained<NSError>>;

        #[cfg(feature = "VNGeometry")]
        #[method_id(@__retain_semantics Other boundingCircleForPoints:error:_)]
        pub unsafe fn boundingCircleForPoints_error(
            points: &NSArray<VNPoint>,
        ) -> Result<Retained<VNCircle>, Retained<NSError>>;

        #[cfg(feature = "VNGeometry")]
        #[method(calculateArea:forContour:orientedArea:error:_)]
        pub unsafe fn calculateArea_forContour_orientedArea_error(
            area: NonNull<c_double>,
            contour: &VNContour,
            oriented_area: bool,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "VNGeometry")]
        #[method(calculatePerimeter:forContour:error:_)]
        pub unsafe fn calculatePerimeter_forContour_error(
            perimeter: NonNull<c_double>,
            contour: &VNContour,
        ) -> Result<(), Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl VNGeometryUtils {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

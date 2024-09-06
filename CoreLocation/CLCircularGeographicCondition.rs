//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CLCondition")]
    pub struct CLCircularGeographicCondition;

    #[cfg(feature = "CLCondition")]
    unsafe impl ClassType for CLCircularGeographicCondition {
        #[inherits(NSObject)]
        type Super = CLCondition;
    }
);

#[cfg(feature = "CLCondition")]
unsafe impl NSCoding for CLCircularGeographicCondition {}

#[cfg(feature = "CLCondition")]
unsafe impl NSCopying for CLCircularGeographicCondition {}

#[cfg(feature = "CLCondition")]
unsafe impl CopyingHelper for CLCircularGeographicCondition {
    type Result = Self;
}

#[cfg(feature = "CLCondition")]
unsafe impl NSObjectProtocol for CLCircularGeographicCondition {}

#[cfg(feature = "CLCondition")]
unsafe impl NSSecureCoding for CLCircularGeographicCondition {}

extern_methods!(
    #[cfg(feature = "CLCondition")]
    unsafe impl CLCircularGeographicCondition {
        #[cfg(feature = "CLLocation")]
        #[method(center)]
        pub unsafe fn center(&self) -> CLLocationCoordinate2D;

        #[cfg(feature = "CLLocation")]
        #[method(radius)]
        pub unsafe fn radius(&self) -> CLLocationDistance;

        #[cfg(feature = "CLLocation")]
        #[method_id(@__retain_semantics Init initWithCenter:radius:)]
        pub unsafe fn initWithCenter_radius(
            this: Allocated<Self>,
            center: CLLocationCoordinate2D,
            radius: CLLocationDistance,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CLCondition`
    #[cfg(feature = "CLCondition")]
    unsafe impl CLCircularGeographicCondition {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

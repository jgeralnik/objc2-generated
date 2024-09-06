//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MKStandardMapEmphasisStyle(pub NSInteger);
impl MKStandardMapEmphasisStyle {
    #[doc(alias = "MKStandardMapEmphasisStyleDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "MKStandardMapEmphasisStyleMuted")]
    pub const Muted: Self = Self(1);
}

unsafe impl Encode for MKStandardMapEmphasisStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MKStandardMapEmphasisStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MKMapConfiguration")]
    pub struct MKStandardMapConfiguration;

    #[cfg(feature = "MKMapConfiguration")]
    unsafe impl ClassType for MKStandardMapConfiguration {
        #[inherits(NSObject)]
        type Super = MKMapConfiguration;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MKMapConfiguration")]
unsafe impl NSCoding for MKStandardMapConfiguration {}

#[cfg(feature = "MKMapConfiguration")]
unsafe impl NSCopying for MKStandardMapConfiguration {}

#[cfg(feature = "MKMapConfiguration")]
unsafe impl CopyingHelper for MKStandardMapConfiguration {
    type Result = Self;
}

#[cfg(feature = "MKMapConfiguration")]
unsafe impl NSObjectProtocol for MKStandardMapConfiguration {}

#[cfg(feature = "MKMapConfiguration")]
unsafe impl NSSecureCoding for MKStandardMapConfiguration {}

extern_methods!(
    #[cfg(feature = "MKMapConfiguration")]
    unsafe impl MKStandardMapConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithElevationStyle:)]
        pub unsafe fn initWithElevationStyle(
            this: Allocated<Self>,
            elevation_style: MKMapElevationStyle,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithElevationStyle:emphasisStyle:)]
        pub unsafe fn initWithElevationStyle_emphasisStyle(
            this: Allocated<Self>,
            elevation_style: MKMapElevationStyle,
            emphasis_style: MKStandardMapEmphasisStyle,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithEmphasisStyle:)]
        pub unsafe fn initWithEmphasisStyle(
            this: Allocated<Self>,
            emphasis_style: MKStandardMapEmphasisStyle,
        ) -> Retained<Self>;

        #[method(emphasisStyle)]
        pub unsafe fn emphasisStyle(&self) -> MKStandardMapEmphasisStyle;

        #[method(setEmphasisStyle:)]
        pub unsafe fn setEmphasisStyle(&self, emphasis_style: MKStandardMapEmphasisStyle);

        #[cfg(feature = "MKPointOfInterestFilter")]
        #[method_id(@__retain_semantics Other pointOfInterestFilter)]
        pub unsafe fn pointOfInterestFilter(&self) -> Option<Retained<MKPointOfInterestFilter>>;

        #[cfg(feature = "MKPointOfInterestFilter")]
        #[method(setPointOfInterestFilter:)]
        pub unsafe fn setPointOfInterestFilter(
            &self,
            point_of_interest_filter: Option<&MKPointOfInterestFilter>,
        );

        #[method(showsTraffic)]
        pub unsafe fn showsTraffic(&self) -> bool;

        #[method(setShowsTraffic:)]
        pub unsafe fn setShowsTraffic(&self, shows_traffic: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `MKMapConfiguration`
    #[cfg(feature = "MKMapConfiguration")]
    unsafe impl MKStandardMapConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

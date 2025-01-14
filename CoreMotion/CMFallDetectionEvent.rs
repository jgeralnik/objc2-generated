//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coremotion/cmfalldetectioneventuserresolution?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CMFallDetectionEventUserResolution(pub NSInteger);
impl CMFallDetectionEventUserResolution {
    #[doc(alias = "CMFallDetectionEventUserResolutionConfirmed")]
    pub const Confirmed: Self = Self(0);
    #[doc(alias = "CMFallDetectionEventUserResolutionDismissed")]
    pub const Dismissed: Self = Self(1);
    #[doc(alias = "CMFallDetectionEventUserResolutionRejected")]
    pub const Rejected: Self = Self(2);
    #[doc(alias = "CMFallDetectionEventUserResolutionUnresponsive")]
    pub const Unresponsive: Self = Self(3);
}

unsafe impl Encode for CMFallDetectionEventUserResolution {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CMFallDetectionEventUserResolution {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coremotion/cmfalldetectionevent?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CMFallDetectionEvent;
);

unsafe impl NSObjectProtocol for CMFallDetectionEvent {}

extern_methods!(
    unsafe impl CMFallDetectionEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other date)]
        pub unsafe fn date(&self) -> Retained<NSDate>;

        #[method(resolution)]
        pub unsafe fn resolution(&self) -> CMFallDetectionEventUserResolution;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CMFallDetectionEvent {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

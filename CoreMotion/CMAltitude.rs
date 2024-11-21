//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(CMLogItem, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CMLogItem")]
    pub struct CMAltitudeData;
);

#[cfg(feature = "CMLogItem")]
unsafe impl NSCoding for CMAltitudeData {}

#[cfg(feature = "CMLogItem")]
unsafe impl NSCopying for CMAltitudeData {}

#[cfg(feature = "CMLogItem")]
unsafe impl CopyingHelper for CMAltitudeData {
    type Result = Self;
}

#[cfg(feature = "CMLogItem")]
unsafe impl NSObjectProtocol for CMAltitudeData {}

#[cfg(feature = "CMLogItem")]
unsafe impl NSSecureCoding for CMAltitudeData {}

extern_methods!(
    #[cfg(feature = "CMLogItem")]
    unsafe impl CMAltitudeData {
        #[method_id(@__retain_semantics Other relativeAltitude)]
        pub unsafe fn relativeAltitude(&self) -> Retained<NSNumber>;

        #[method_id(@__retain_semantics Other pressure)]
        pub unsafe fn pressure(&self) -> Retained<NSNumber>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CMLogItem")]
    unsafe impl CMAltitudeData {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZGraphicsDeviceConfiguration")]
    pub struct VZVirtioGraphicsDeviceConfiguration;

    #[cfg(feature = "VZGraphicsDeviceConfiguration")]
    unsafe impl ClassType for VZVirtioGraphicsDeviceConfiguration {
        #[inherits(NSObject)]
        type Super = VZGraphicsDeviceConfiguration;
    }
);

#[cfg(feature = "VZGraphicsDeviceConfiguration")]
unsafe impl NSCopying for VZVirtioGraphicsDeviceConfiguration {}

#[cfg(feature = "VZGraphicsDeviceConfiguration")]
unsafe impl CopyingHelper for VZVirtioGraphicsDeviceConfiguration {
    type Result = Self;
}

#[cfg(feature = "VZGraphicsDeviceConfiguration")]
unsafe impl NSObjectProtocol for VZVirtioGraphicsDeviceConfiguration {}

extern_methods!(
    #[cfg(feature = "VZGraphicsDeviceConfiguration")]
    unsafe impl VZVirtioGraphicsDeviceConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(
            feature = "VZGraphicsDisplayConfiguration",
            feature = "VZVirtioGraphicsScanoutConfiguration"
        ))]
        #[method_id(@__retain_semantics Other scanouts)]
        pub unsafe fn scanouts(&self) -> Retained<NSArray<VZVirtioGraphicsScanoutConfiguration>>;

        #[cfg(all(
            feature = "VZGraphicsDisplayConfiguration",
            feature = "VZVirtioGraphicsScanoutConfiguration"
        ))]
        #[method(setScanouts:)]
        pub unsafe fn setScanouts(&self, scanouts: &NSArray<VZVirtioGraphicsScanoutConfiguration>);
    }
);

extern_methods!(
    /// Methods declared on superclass `VZGraphicsDeviceConfiguration`
    #[cfg(feature = "VZGraphicsDeviceConfiguration")]
    unsafe impl VZVirtioGraphicsDeviceConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

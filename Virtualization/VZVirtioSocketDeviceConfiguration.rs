//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzvirtiosocketdeviceconfiguration?language=objc)
    #[unsafe(super(VZSocketDeviceConfiguration, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZSocketDeviceConfiguration")]
    pub struct VZVirtioSocketDeviceConfiguration;
);

#[cfg(feature = "VZSocketDeviceConfiguration")]
unsafe impl NSCopying for VZVirtioSocketDeviceConfiguration {}

#[cfg(feature = "VZSocketDeviceConfiguration")]
unsafe impl CopyingHelper for VZVirtioSocketDeviceConfiguration {
    type Result = Self;
}

#[cfg(feature = "VZSocketDeviceConfiguration")]
unsafe impl NSObjectProtocol for VZVirtioSocketDeviceConfiguration {}

extern_methods!(
    #[cfg(feature = "VZSocketDeviceConfiguration")]
    unsafe impl VZVirtioSocketDeviceConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZSocketDeviceConfiguration`
    #[cfg(feature = "VZSocketDeviceConfiguration")]
    unsafe impl VZVirtioSocketDeviceConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

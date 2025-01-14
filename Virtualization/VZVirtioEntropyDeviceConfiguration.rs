//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzvirtioentropydeviceconfiguration?language=objc)
    #[unsafe(super(VZEntropyDeviceConfiguration, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZEntropyDeviceConfiguration")]
    pub struct VZVirtioEntropyDeviceConfiguration;
);

#[cfg(feature = "VZEntropyDeviceConfiguration")]
unsafe impl NSCopying for VZVirtioEntropyDeviceConfiguration {}

#[cfg(feature = "VZEntropyDeviceConfiguration")]
unsafe impl CopyingHelper for VZVirtioEntropyDeviceConfiguration {
    type Result = Self;
}

#[cfg(feature = "VZEntropyDeviceConfiguration")]
unsafe impl NSObjectProtocol for VZVirtioEntropyDeviceConfiguration {}

extern_methods!(
    #[cfg(feature = "VZEntropyDeviceConfiguration")]
    unsafe impl VZVirtioEntropyDeviceConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZEntropyDeviceConfiguration`
    #[cfg(feature = "VZEntropyDeviceConfiguration")]
    unsafe impl VZVirtioEntropyDeviceConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZStorageDeviceConfiguration")]
    pub struct VZUSBMassStorageDeviceConfiguration;

    #[cfg(feature = "VZStorageDeviceConfiguration")]
    unsafe impl ClassType for VZUSBMassStorageDeviceConfiguration {
        #[inherits(NSObject)]
        type Super = VZStorageDeviceConfiguration;
    }
);

#[cfg(feature = "VZStorageDeviceConfiguration")]
unsafe impl NSCopying for VZUSBMassStorageDeviceConfiguration {}

#[cfg(feature = "VZStorageDeviceConfiguration")]
unsafe impl CopyingHelper for VZUSBMassStorageDeviceConfiguration {
    type Result = Self;
}

#[cfg(feature = "VZStorageDeviceConfiguration")]
unsafe impl NSObjectProtocol for VZUSBMassStorageDeviceConfiguration {}

extern_methods!(
    #[cfg(feature = "VZStorageDeviceConfiguration")]
    unsafe impl VZUSBMassStorageDeviceConfiguration {
        #[cfg(feature = "VZStorageDeviceAttachment")]
        #[method_id(@__retain_semantics Init initWithAttachment:)]
        pub unsafe fn initWithAttachment(
            this: Allocated<Self>,
            attachment: &VZStorageDeviceAttachment,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZStorageDeviceConfiguration`
    #[cfg(feature = "VZStorageDeviceConfiguration")]
    unsafe impl VZUSBMassStorageDeviceConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

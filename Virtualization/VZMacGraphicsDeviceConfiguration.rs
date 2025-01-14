//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzmacgraphicsdeviceconfiguration?language=objc)
    #[unsafe(super(VZGraphicsDeviceConfiguration, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZGraphicsDeviceConfiguration")]
    pub struct VZMacGraphicsDeviceConfiguration;
);

#[cfg(feature = "VZGraphicsDeviceConfiguration")]
unsafe impl NSCopying for VZMacGraphicsDeviceConfiguration {}

#[cfg(feature = "VZGraphicsDeviceConfiguration")]
unsafe impl CopyingHelper for VZMacGraphicsDeviceConfiguration {
    type Result = Self;
}

#[cfg(feature = "VZGraphicsDeviceConfiguration")]
unsafe impl NSObjectProtocol for VZMacGraphicsDeviceConfiguration {}

extern_methods!(
    #[cfg(feature = "VZGraphicsDeviceConfiguration")]
    unsafe impl VZMacGraphicsDeviceConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(
            feature = "VZGraphicsDisplayConfiguration",
            feature = "VZMacGraphicsDisplayConfiguration"
        ))]
        #[method_id(@__retain_semantics Other displays)]
        pub unsafe fn displays(&self) -> Retained<NSArray<VZMacGraphicsDisplayConfiguration>>;

        #[cfg(all(
            feature = "VZGraphicsDisplayConfiguration",
            feature = "VZMacGraphicsDisplayConfiguration"
        ))]
        #[method(setDisplays:)]
        pub unsafe fn setDisplays(&self, displays: &NSArray<VZMacGraphicsDisplayConfiguration>);
    }
);

extern_methods!(
    /// Methods declared on superclass `VZGraphicsDeviceConfiguration`
    #[cfg(feature = "VZGraphicsDeviceConfiguration")]
    unsafe impl VZMacGraphicsDeviceConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

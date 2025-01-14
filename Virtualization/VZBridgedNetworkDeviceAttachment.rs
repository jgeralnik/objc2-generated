//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzbridgednetworkdeviceattachment?language=objc)
    #[unsafe(super(VZNetworkDeviceAttachment, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZNetworkDeviceAttachment")]
    pub struct VZBridgedNetworkDeviceAttachment;
);

#[cfg(feature = "VZNetworkDeviceAttachment")]
unsafe impl NSObjectProtocol for VZBridgedNetworkDeviceAttachment {}

extern_methods!(
    #[cfg(feature = "VZNetworkDeviceAttachment")]
    unsafe impl VZBridgedNetworkDeviceAttachment {
        #[cfg(feature = "VZBridgedNetworkInterface")]
        #[method_id(@__retain_semantics Init initWithInterface:)]
        pub unsafe fn initWithInterface(
            this: Allocated<Self>,
            interface: &VZBridgedNetworkInterface,
        ) -> Retained<Self>;

        #[cfg(feature = "VZBridgedNetworkInterface")]
        #[method_id(@__retain_semantics Other interface)]
        pub unsafe fn interface(&self) -> Retained<VZBridgedNetworkInterface>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZNetworkDeviceAttachment`
    #[cfg(feature = "VZNetworkDeviceAttachment")]
    unsafe impl VZBridgedNetworkDeviceAttachment {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/virtualization/vznetworkdevice?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZNetworkDevice;
);

unsafe impl NSObjectProtocol for VZNetworkDevice {}

extern_methods!(
    unsafe impl VZNetworkDevice {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "VZNetworkDeviceAttachment")]
        #[method_id(@__retain_semantics Other attachment)]
        pub unsafe fn attachment(&self) -> Option<Retained<VZNetworkDeviceAttachment>>;

        #[cfg(feature = "VZNetworkDeviceAttachment")]
        #[method(setAttachment:)]
        pub unsafe fn setAttachment(&self, attachment: Option<&VZNetworkDeviceAttachment>);
    }
);

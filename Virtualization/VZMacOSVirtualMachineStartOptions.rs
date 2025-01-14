//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzmacosvirtualmachinestartoptions?language=objc)
    #[unsafe(super(VZVirtualMachineStartOptions, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZVirtualMachineStartOptions")]
    pub struct VZMacOSVirtualMachineStartOptions;
);

#[cfg(feature = "VZVirtualMachineStartOptions")]
unsafe impl NSObjectProtocol for VZMacOSVirtualMachineStartOptions {}

extern_methods!(
    #[cfg(feature = "VZVirtualMachineStartOptions")]
    unsafe impl VZMacOSVirtualMachineStartOptions {
        #[method(startUpFromMacOSRecovery)]
        pub unsafe fn startUpFromMacOSRecovery(&self) -> bool;

        #[method(setStartUpFromMacOSRecovery:)]
        pub unsafe fn setStartUpFromMacOSRecovery(&self, start_up_from_mac_os_recovery: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "VZVirtualMachineStartOptions")]
    unsafe impl VZMacOSVirtualMachineStartOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

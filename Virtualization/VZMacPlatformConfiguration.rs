//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZPlatformConfiguration")]
    pub struct VZMacPlatformConfiguration;

    #[cfg(feature = "VZPlatformConfiguration")]
    unsafe impl ClassType for VZMacPlatformConfiguration {
        #[inherits(NSObject)]
        type Super = VZPlatformConfiguration;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "VZPlatformConfiguration")]
unsafe impl NSCopying for VZMacPlatformConfiguration {}

#[cfg(feature = "VZPlatformConfiguration")]
unsafe impl NSObjectProtocol for VZMacPlatformConfiguration {}

extern_methods!(
    #[cfg(feature = "VZPlatformConfiguration")]
    unsafe impl VZMacPlatformConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "VZMacHardwareModel")]
        #[method_id(@__retain_semantics Other hardwareModel)]
        pub unsafe fn hardwareModel(&self) -> Id<VZMacHardwareModel>;

        #[cfg(feature = "VZMacHardwareModel")]
        #[method(setHardwareModel:)]
        pub unsafe fn setHardwareModel(&self, hardware_model: &VZMacHardwareModel);

        #[cfg(feature = "VZMacMachineIdentifier")]
        #[method_id(@__retain_semantics Other machineIdentifier)]
        pub unsafe fn machineIdentifier(&self) -> Id<VZMacMachineIdentifier>;

        #[cfg(feature = "VZMacMachineIdentifier")]
        #[method(setMachineIdentifier:)]
        pub unsafe fn setMachineIdentifier(&self, machine_identifier: &VZMacMachineIdentifier);

        #[cfg(feature = "VZMacAuxiliaryStorage")]
        #[method_id(@__retain_semantics Other auxiliaryStorage)]
        pub unsafe fn auxiliaryStorage(&self) -> Option<Id<VZMacAuxiliaryStorage>>;

        #[cfg(feature = "VZMacAuxiliaryStorage")]
        #[method(setAuxiliaryStorage:)]
        pub unsafe fn setAuxiliaryStorage(&self, auxiliary_storage: Option<&VZMacAuxiliaryStorage>);
    }
);

extern_methods!(
    /// Methods declared on superclass `VZPlatformConfiguration`
    #[cfg(feature = "VZPlatformConfiguration")]
    unsafe impl VZMacPlatformConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
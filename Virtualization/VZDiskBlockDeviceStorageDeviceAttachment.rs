//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZStorageDeviceAttachment")]
    pub struct VZDiskBlockDeviceStorageDeviceAttachment;

    #[cfg(feature = "VZStorageDeviceAttachment")]
    unsafe impl ClassType for VZDiskBlockDeviceStorageDeviceAttachment {
        #[inherits(NSObject)]
        type Super = VZStorageDeviceAttachment;
    }
);

#[cfg(feature = "VZStorageDeviceAttachment")]
unsafe impl NSObjectProtocol for VZDiskBlockDeviceStorageDeviceAttachment {}

extern_methods!(
    #[cfg(feature = "VZStorageDeviceAttachment")]
    unsafe impl VZDiskBlockDeviceStorageDeviceAttachment {
        #[cfg(feature = "VZDiskSynchronizationMode")]
        #[method_id(@__retain_semantics Init initWithFileHandle:readOnly:synchronizationMode:error:_)]
        pub unsafe fn initWithFileHandle_readOnly_synchronizationMode_error(
            this: Allocated<Self>,
            file_handle: &NSFileHandle,
            read_only: bool,
            synchronization_mode: VZDiskSynchronizationMode,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method_id(@__retain_semantics Other fileHandle)]
        pub unsafe fn fileHandle(&self) -> Retained<NSFileHandle>;

        #[method(isReadOnly)]
        pub unsafe fn isReadOnly(&self) -> bool;

        #[cfg(feature = "VZDiskSynchronizationMode")]
        #[method(synchronizationMode)]
        pub unsafe fn synchronizationMode(&self) -> VZDiskSynchronizationMode;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZStorageDeviceAttachment`
    #[cfg(feature = "VZStorageDeviceAttachment")]
    unsafe impl VZDiskBlockDeviceStorageDeviceAttachment {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

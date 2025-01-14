//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzdiskimagecachingmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VZDiskImageCachingMode(pub NSInteger);
impl VZDiskImageCachingMode {
    #[doc(alias = "VZDiskImageCachingModeAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "VZDiskImageCachingModeUncached")]
    pub const Uncached: Self = Self(1);
    #[doc(alias = "VZDiskImageCachingModeCached")]
    pub const Cached: Self = Self(2);
}

unsafe impl Encode for VZDiskImageCachingMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for VZDiskImageCachingMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzdiskimagesynchronizationmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VZDiskImageSynchronizationMode(pub NSInteger);
impl VZDiskImageSynchronizationMode {
    #[doc(alias = "VZDiskImageSynchronizationModeFull")]
    pub const Full: Self = Self(1);
    #[doc(alias = "VZDiskImageSynchronizationModeFsync")]
    pub const Fsync: Self = Self(2);
    #[doc(alias = "VZDiskImageSynchronizationModeNone")]
    pub const None: Self = Self(3);
}

unsafe impl Encode for VZDiskImageSynchronizationMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for VZDiskImageSynchronizationMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzdiskimagestoragedeviceattachment?language=objc)
    #[unsafe(super(VZStorageDeviceAttachment, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZStorageDeviceAttachment")]
    pub struct VZDiskImageStorageDeviceAttachment;
);

#[cfg(feature = "VZStorageDeviceAttachment")]
unsafe impl NSObjectProtocol for VZDiskImageStorageDeviceAttachment {}

extern_methods!(
    #[cfg(feature = "VZStorageDeviceAttachment")]
    unsafe impl VZDiskImageStorageDeviceAttachment {
        #[method_id(@__retain_semantics Init initWithURL:readOnly:error:_)]
        pub unsafe fn initWithURL_readOnly_error(
            this: Allocated<Self>,
            url: &NSURL,
            read_only: bool,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method_id(@__retain_semantics Init initWithURL:readOnly:cachingMode:synchronizationMode:error:_)]
        pub unsafe fn initWithURL_readOnly_cachingMode_synchronizationMode_error(
            this: Allocated<Self>,
            url: &NSURL,
            read_only: bool,
            caching_mode: VZDiskImageCachingMode,
            synchronization_mode: VZDiskImageSynchronizationMode,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Retained<NSURL>;

        #[method(isReadOnly)]
        pub unsafe fn isReadOnly(&self) -> bool;

        #[method(cachingMode)]
        pub unsafe fn cachingMode(&self) -> VZDiskImageCachingMode;

        #[method(synchronizationMode)]
        pub unsafe fn synchronizationMode(&self) -> VZDiskImageSynchronizationMode;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZStorageDeviceAttachment`
    #[cfg(feature = "VZStorageDeviceAttachment")]
    unsafe impl VZDiskImageStorageDeviceAttachment {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

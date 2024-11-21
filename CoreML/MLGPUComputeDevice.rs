//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-metal")]
#[cfg(not(target_os = "watchos"))]
use objc2_metal::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLGPUComputeDevice;
);

unsafe impl Send for MLGPUComputeDevice {}

unsafe impl Sync for MLGPUComputeDevice {}

#[cfg(feature = "MLComputeDeviceProtocol")]
unsafe impl MLComputeDeviceProtocol for MLGPUComputeDevice {}

unsafe impl NSObjectProtocol for MLGPUComputeDevice {}

extern_methods!(
    unsafe impl MLGPUComputeDevice {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "objc2-metal")]
        #[cfg(not(target_os = "watchos"))]
        #[method_id(@__retain_semantics Other metalDevice)]
        pub unsafe fn metalDevice(&self) -> Option<Retained<ProtocolObject<dyn MTLDevice>>>;
    }
);

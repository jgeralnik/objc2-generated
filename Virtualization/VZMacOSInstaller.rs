//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzmacosinstaller?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZMacOSInstaller;
);

unsafe impl NSObjectProtocol for VZMacOSInstaller {}

extern_methods!(
    unsafe impl VZMacOSInstaller {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "VZVirtualMachine")]
        #[method_id(@__retain_semantics Init initWithVirtualMachine:restoreImageURL:)]
        pub unsafe fn initWithVirtualMachine_restoreImageURL(
            this: Allocated<Self>,
            virtual_machine: &VZVirtualMachine,
            restore_image_file_url: &NSURL,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method(installWithCompletionHandler:)]
        pub unsafe fn installWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[method_id(@__retain_semantics Other progress)]
        pub unsafe fn progress(&self) -> Retained<NSProgress>;

        #[cfg(feature = "VZVirtualMachine")]
        #[method_id(@__retain_semantics Other virtualMachine)]
        pub unsafe fn virtualMachine(&self) -> Retained<VZVirtualMachine>;

        #[method_id(@__retain_semantics Other restoreImageURL)]
        pub unsafe fn restoreImageURL(&self) -> Retained<NSURL>;
    }
);

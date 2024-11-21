//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZUSBController;
);

unsafe impl NSObjectProtocol for VZUSBController {}

extern_methods!(
    unsafe impl VZUSBController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(feature = "VZUSBDevice", feature = "block2"))]
        #[method(attachDevice:completionHandler:)]
        pub unsafe fn attachDevice_completionHandler(
            &self,
            device: &ProtocolObject<dyn VZUSBDevice>,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(feature = "VZUSBDevice", feature = "block2"))]
        #[method(detachDevice:completionHandler:)]
        pub unsafe fn detachDevice_completionHandler(
            &self,
            device: &ProtocolObject<dyn VZUSBDevice>,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "VZUSBDevice")]
        #[method_id(@__retain_semantics Other usbDevices)]
        pub unsafe fn usbDevices(&self) -> Retained<NSArray<ProtocolObject<dyn VZUSBDevice>>>;
    }
);

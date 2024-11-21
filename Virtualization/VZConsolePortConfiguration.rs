//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZConsolePortConfiguration;
);

unsafe impl NSCopying for VZConsolePortConfiguration {}

unsafe impl CopyingHelper for VZConsolePortConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for VZConsolePortConfiguration {}

extern_methods!(
    unsafe impl VZConsolePortConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "VZSerialPortAttachment")]
        #[method_id(@__retain_semantics Other attachment)]
        pub unsafe fn attachment(&self) -> Option<Retained<VZSerialPortAttachment>>;

        #[cfg(feature = "VZSerialPortAttachment")]
        #[method(setAttachment:)]
        pub unsafe fn setAttachment(&self, attachment: Option<&VZSerialPortAttachment>);
    }
);

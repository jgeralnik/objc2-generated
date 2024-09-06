//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZSerialPortAttachment")]
    pub struct VZSpiceAgentPortAttachment;

    #[cfg(feature = "VZSerialPortAttachment")]
    unsafe impl ClassType for VZSpiceAgentPortAttachment {
        #[inherits(NSObject)]
        type Super = VZSerialPortAttachment;
    }
);

#[cfg(feature = "VZSerialPortAttachment")]
unsafe impl NSObjectProtocol for VZSpiceAgentPortAttachment {}

extern_methods!(
    #[cfg(feature = "VZSerialPortAttachment")]
    unsafe impl VZSpiceAgentPortAttachment {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(sharesClipboard)]
        pub unsafe fn sharesClipboard(&self) -> bool;

        #[method(setSharesClipboard:)]
        pub unsafe fn setSharesClipboard(&self, shares_clipboard: bool);

        #[method_id(@__retain_semantics Other spiceAgentPortName)]
        pub unsafe fn spiceAgentPortName() -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZSerialPortAttachment`
    #[cfg(feature = "VZSerialPortAttachment")]
    unsafe impl VZSpiceAgentPortAttachment {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

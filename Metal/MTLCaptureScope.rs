//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait MTLCaptureScope: NSObjectProtocol + IsRetainable {
        #[method(beginScope)]
        fn beginScope(&self);

        #[method(endScope)]
        fn endScope(&self);

        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Id<NSString>>;

        #[method(setLabel:)]
        unsafe fn setLabel(&self, label: Option<&NSString>);

        #[cfg(feature = "MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        unsafe fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>>;

        #[cfg(feature = "MTLCommandQueue")]
        #[method_id(@__retain_semantics Other commandQueue)]
        unsafe fn commandQueue(&self) -> Option<Id<ProtocolObject<dyn MTLCommandQueue>>>;
    }

    unsafe impl ProtocolType for dyn MTLCaptureScope {}
);

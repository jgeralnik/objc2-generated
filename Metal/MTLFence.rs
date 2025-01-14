//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlfence?language=objc)
    pub unsafe trait MTLFence: NSObjectProtocol {
        #[cfg(feature = "MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Retained<NSString>>;

        #[method(setLabel:)]
        fn setLabel(&self, label: Option<&NSString>);
    }

    unsafe impl ProtocolType for dyn MTLFence {}
);

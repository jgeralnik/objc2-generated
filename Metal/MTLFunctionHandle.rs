//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait MTLFunctionHandle: NSObjectProtocol {
        #[cfg(feature = "MTLLibrary")]
        #[method(functionType)]
        fn functionType(&self) -> MTLFunctionType;

        #[method_id(@__retain_semantics Other name)]
        fn name(&self) -> Retained<NSString>;

        #[cfg(feature = "MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;
    }

    unsafe impl ProtocolType for dyn MTLFunctionHandle {}
);

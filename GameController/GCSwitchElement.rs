//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcswitchelement?language=objc)
    #[cfg(feature = "GCPhysicalInputElement")]
    pub unsafe trait GCSwitchElement: GCPhysicalInputElement {
        #[cfg(feature = "GCSwitchPositionInput")]
        #[method_id(@__retain_semantics Other positionInput)]
        unsafe fn positionInput(&self) -> Retained<ProtocolObject<dyn GCSwitchPositionInput>>;
    }

    #[cfg(feature = "GCPhysicalInputElement")]
    unsafe impl ProtocolType for dyn GCSwitchElement {}
);

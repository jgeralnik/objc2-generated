//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcbuttonelement?language=objc)
    #[cfg(feature = "GCPhysicalInputElement")]
    pub unsafe trait GCButtonElement: GCPhysicalInputElement {
        #[cfg(all(feature = "GCLinearInput", feature = "GCPressedStateInput"))]
        #[method_id(@__retain_semantics Other pressedInput)]
        unsafe fn pressedInput(
            &self,
        ) -> Retained<AnyObject /* GCPressedStateInput+ GCLinearInput */>;

        #[cfg(feature = "GCTouchedStateInput")]
        #[method_id(@__retain_semantics Other touchedInput)]
        unsafe fn touchedInput(&self) -> Option<Retained<ProtocolObject<dyn GCTouchedStateInput>>>;
    }

    #[cfg(feature = "GCPhysicalInputElement")]
    unsafe impl ProtocolType for dyn GCButtonElement {}
);

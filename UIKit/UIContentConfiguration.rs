//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicontentconfiguration?language=objc)
    pub unsafe trait UIContentConfiguration:
        NSCopying + NSObjectProtocol + MainThreadOnly
    {
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other makeContentView)]
        unsafe fn makeContentView(&self) -> Retained<UIView>;

        #[cfg(feature = "UIConfigurationState")]
        #[method_id(@__retain_semantics Other updatedConfigurationForState:)]
        unsafe fn updatedConfigurationForState(
            &self,
            state: &ProtocolObject<dyn UIConfigurationState>,
        ) -> Retained<Self>;
    }

    unsafe impl ProtocolType for dyn UIContentConfiguration {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicontentview?language=objc)
    pub unsafe trait UIContentView: NSObjectProtocol + MainThreadOnly {
        #[method_id(@__retain_semantics Other configuration)]
        unsafe fn configuration(&self) -> Retained<ProtocolObject<dyn UIContentConfiguration>>;

        #[method(setConfiguration:)]
        unsafe fn setConfiguration(
            &self,
            configuration: &ProtocolObject<dyn UIContentConfiguration>,
        );

        #[optional]
        #[method(supportsConfiguration:)]
        unsafe fn supportsConfiguration(
            &self,
            configuration: &ProtocolObject<dyn UIContentConfiguration>,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn UIContentView {}
);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uibehavioralstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIBehavioralStyle(pub NSUInteger);
impl UIBehavioralStyle {
    #[doc(alias = "UIBehavioralStyleAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "UIBehavioralStylePad")]
    pub const Pad: Self = Self(1);
    #[doc(alias = "UIBehavioralStyleMac")]
    pub const Mac: Self = Self(2);
}

unsafe impl Encode for UIBehavioralStyle {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIBehavioralStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// UIBehavioralStyle
    #[cfg(all(
        feature = "UIButton",
        feature = "UIControl",
        feature = "UIResponder",
        feature = "UIView"
    ))]
    unsafe impl UIButton {
        #[method(behavioralStyle)]
        pub unsafe fn behavioralStyle(&self) -> UIBehavioralStyle;

        #[method(preferredBehavioralStyle)]
        pub unsafe fn preferredBehavioralStyle(&self) -> UIBehavioralStyle;

        #[method(setPreferredBehavioralStyle:)]
        pub unsafe fn setPreferredBehavioralStyle(
            &self,
            preferred_behavioral_style: UIBehavioralStyle,
        );
    }
);

extern_methods!(
    /// UIBehavioralStyle
    #[cfg(all(
        feature = "UIControl",
        feature = "UIResponder",
        feature = "UISlider",
        feature = "UIView"
    ))]
    unsafe impl UISlider {
        #[method(behavioralStyle)]
        pub unsafe fn behavioralStyle(&self) -> UIBehavioralStyle;

        #[method(preferredBehavioralStyle)]
        pub unsafe fn preferredBehavioralStyle(&self) -> UIBehavioralStyle;

        #[method(setPreferredBehavioralStyle:)]
        pub unsafe fn setPreferredBehavioralStyle(
            &self,
            preferred_behavioral_style: UIBehavioralStyle,
        );
    }
);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollercomponentkey?language=objc)
// NS_TYPED_ENUM
pub type UITextFormattingViewControllerComponentKey = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollerformattingstylescomponentkey?language=objc)
    pub static UITextFormattingViewControllerFormattingStylesComponentKey:
        &'static UITextFormattingViewControllerComponentKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollerfontattributescomponentkey?language=objc)
    pub static UITextFormattingViewControllerFontAttributesComponentKey:
        &'static UITextFormattingViewControllerComponentKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollerfontpickercomponentkey?language=objc)
    pub static UITextFormattingViewControllerFontPickerComponentKey:
        &'static UITextFormattingViewControllerComponentKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollerfontsizecomponentkey?language=objc)
    pub static UITextFormattingViewControllerFontSizeComponentKey:
        &'static UITextFormattingViewControllerComponentKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollerfontpointsizecomponentkey?language=objc)
    pub static UITextFormattingViewControllerFontPointSizeComponentKey:
        &'static UITextFormattingViewControllerComponentKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollertextalignmentcomponentkey?language=objc)
    pub static UITextFormattingViewControllerTextAlignmentComponentKey:
        &'static UITextFormattingViewControllerComponentKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollertextalignmentandjustificationcomponentkey?language=objc)
    pub static UITextFormattingViewControllerTextAlignmentAndJustificationComponentKey:
        &'static UITextFormattingViewControllerComponentKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollertextindentationcomponentkey?language=objc)
    pub static UITextFormattingViewControllerTextIndentationComponentKey:
        &'static UITextFormattingViewControllerComponentKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollerlineheightcomponentkey?language=objc)
    pub static UITextFormattingViewControllerLineHeightComponentKey:
        &'static UITextFormattingViewControllerComponentKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollerliststylescomponentkey?language=objc)
    pub static UITextFormattingViewControllerListStylesComponentKey:
        &'static UITextFormattingViewControllerComponentKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollertextcolorcomponentkey?language=objc)
    pub static UITextFormattingViewControllerTextColorComponentKey:
        &'static UITextFormattingViewControllerComponentKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollerhighlightcomponentkey?language=objc)
    pub static UITextFormattingViewControllerHighlightComponentKey:
        &'static UITextFormattingViewControllerComponentKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollerhighlightpickercomponentkey?language=objc)
    pub static UITextFormattingViewControllerHighlightPickerComponentKey:
        &'static UITextFormattingViewControllerComponentKey;
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollercomponentsize?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITextFormattingViewControllerComponentSize(pub NSInteger);
impl UITextFormattingViewControllerComponentSize {
    #[doc(alias = "UITextFormattingViewControllerComponentSizeAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "UITextFormattingViewControllerComponentSizeMini")]
    pub const Mini: Self = Self(1);
    #[doc(alias = "UITextFormattingViewControllerComponentSizeSmall")]
    pub const Small: Self = Self(2);
    #[doc(alias = "UITextFormattingViewControllerComponentSizeRegular")]
    pub const Regular: Self = Self(3);
    #[doc(alias = "UITextFormattingViewControllerComponentSizeLarge")]
    pub const Large: Self = Self(4);
    #[doc(alias = "UITextFormattingViewControllerComponentSizeExtraLarge")]
    pub const ExtraLarge: Self = Self(5);
}

unsafe impl Encode for UITextFormattingViewControllerComponentSize {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITextFormattingViewControllerComponentSize {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollercomponent?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextFormattingViewControllerComponent;
);

unsafe impl NSCoding for UITextFormattingViewControllerComponent {}

unsafe impl NSCopying for UITextFormattingViewControllerComponent {}

unsafe impl CopyingHelper for UITextFormattingViewControllerComponent {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UITextFormattingViewControllerComponent {}

unsafe impl NSSecureCoding for UITextFormattingViewControllerComponent {}

extern_methods!(
    unsafe impl UITextFormattingViewControllerComponent {
        #[method_id(@__retain_semantics Other componentKey)]
        pub unsafe fn componentKey(&self) -> Retained<UITextFormattingViewControllerComponentKey>;

        #[method(preferredSize)]
        pub unsafe fn preferredSize(&self) -> UITextFormattingViewControllerComponentSize;

        #[method_id(@__retain_semantics Init initWithComponentKey:preferredSize:)]
        pub unsafe fn initWithComponentKey_preferredSize(
            this: Allocated<Self>,
            component_key: &UITextFormattingViewControllerComponentKey,
            preferred_size: UITextFormattingViewControllerComponentSize,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollercomponentgroup?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextFormattingViewControllerComponentGroup;
);

unsafe impl NSCoding for UITextFormattingViewControllerComponentGroup {}

unsafe impl NSCopying for UITextFormattingViewControllerComponentGroup {}

unsafe impl CopyingHelper for UITextFormattingViewControllerComponentGroup {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UITextFormattingViewControllerComponentGroup {}

unsafe impl NSSecureCoding for UITextFormattingViewControllerComponentGroup {}

extern_methods!(
    unsafe impl UITextFormattingViewControllerComponentGroup {
        #[method_id(@__retain_semantics Other components)]
        pub unsafe fn components(
            &self,
        ) -> Retained<NSArray<UITextFormattingViewControllerComponent>>;

        #[method_id(@__retain_semantics Init initWithComponents:)]
        pub unsafe fn initWithComponents(
            this: Allocated<Self>,
            components: &NSArray<UITextFormattingViewControllerComponent>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

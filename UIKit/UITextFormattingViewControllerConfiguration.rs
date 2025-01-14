//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollerconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextFormattingViewControllerConfiguration;
);

unsafe impl NSCoding for UITextFormattingViewControllerConfiguration {}

unsafe impl NSCopying for UITextFormattingViewControllerConfiguration {}

unsafe impl CopyingHelper for UITextFormattingViewControllerConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UITextFormattingViewControllerConfiguration {}

unsafe impl NSSecureCoding for UITextFormattingViewControllerConfiguration {}

extern_methods!(
    unsafe impl UITextFormattingViewControllerConfiguration {
        #[cfg(feature = "UITextFormattingViewControllerComponent")]
        #[method_id(@__retain_semantics Other groups)]
        pub unsafe fn groups(
            &self,
        ) -> Retained<NSArray<UITextFormattingViewControllerComponentGroup>>;

        #[cfg(feature = "UITextFormattingViewControllerFormattingStyle")]
        #[method_id(@__retain_semantics Other formattingStyles)]
        pub unsafe fn formattingStyles(
            &self,
        ) -> Option<Retained<NSArray<UITextFormattingViewControllerFormattingStyle>>>;

        #[cfg(feature = "UITextFormattingViewControllerFormattingStyle")]
        #[method(setFormattingStyles:)]
        pub unsafe fn setFormattingStyles(
            &self,
            formatting_styles: Option<&NSArray<UITextFormattingViewControllerFormattingStyle>>,
        );

        #[cfg(feature = "UIFontPickerViewControllerConfiguration")]
        #[method_id(@__retain_semantics Other fontPickerConfiguration)]
        pub unsafe fn fontPickerConfiguration(
            &self,
            mtm: MainThreadMarker,
        ) -> Option<Retained<UIFontPickerViewControllerConfiguration>>;

        #[cfg(feature = "UIFontPickerViewControllerConfiguration")]
        #[method(setFontPickerConfiguration:)]
        pub unsafe fn setFontPickerConfiguration(
            &self,
            font_picker_configuration: Option<&UIFontPickerViewControllerConfiguration>,
        );

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "UITextFormattingViewControllerComponent")]
        #[method_id(@__retain_semantics Init initWithGroups:)]
        pub unsafe fn initWithGroups(
            this: Allocated<Self>,
            groups: &NSArray<UITextFormattingViewControllerComponentGroup>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITextFormattingViewControllerConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

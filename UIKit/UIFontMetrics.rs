//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIFontMetrics;
);

unsafe impl Send for UIFontMetrics {}

unsafe impl Sync for UIFontMetrics {}

unsafe impl NSObjectProtocol for UIFontMetrics {}

extern_methods!(
    unsafe impl UIFontMetrics {
        #[method_id(@__retain_semantics Other defaultMetrics)]
        pub unsafe fn defaultMetrics() -> Retained<UIFontMetrics>;

        #[cfg(feature = "UIFontDescriptor")]
        #[method_id(@__retain_semantics Other metricsForTextStyle:)]
        pub unsafe fn metricsForTextStyle(text_style: &UIFontTextStyle) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "UIFontDescriptor")]
        #[method_id(@__retain_semantics Init initForTextStyle:)]
        pub unsafe fn initForTextStyle(
            this: Allocated<Self>,
            text_style: &UIFontTextStyle,
        ) -> Retained<Self>;

        #[cfg(feature = "UIFont")]
        #[method_id(@__retain_semantics Other scaledFontForFont:)]
        pub unsafe fn scaledFontForFont(&self, font: &UIFont) -> Retained<UIFont>;

        #[cfg(feature = "UIFont")]
        #[method_id(@__retain_semantics Other scaledFontForFont:maximumPointSize:)]
        pub unsafe fn scaledFontForFont_maximumPointSize(
            &self,
            font: &UIFont,
            maximum_point_size: CGFloat,
        ) -> Retained<UIFont>;

        #[cfg(all(feature = "UIFont", feature = "UITraitCollection"))]
        #[method_id(@__retain_semantics Other scaledFontForFont:compatibleWithTraitCollection:)]
        pub unsafe fn scaledFontForFont_compatibleWithTraitCollection(
            &self,
            font: &UIFont,
            trait_collection: Option<&UITraitCollection>,
        ) -> Retained<UIFont>;

        #[cfg(all(feature = "UIFont", feature = "UITraitCollection"))]
        #[method_id(@__retain_semantics Other scaledFontForFont:maximumPointSize:compatibleWithTraitCollection:)]
        pub unsafe fn scaledFontForFont_maximumPointSize_compatibleWithTraitCollection(
            &self,
            font: &UIFont,
            maximum_point_size: CGFloat,
            trait_collection: Option<&UITraitCollection>,
        ) -> Retained<UIFont>;

        #[method(scaledValueForValue:)]
        pub unsafe fn scaledValueForValue(&self, value: CGFloat) -> CGFloat;

        #[cfg(feature = "UITraitCollection")]
        #[method(scaledValueForValue:compatibleWithTraitCollection:)]
        pub unsafe fn scaledValueForValue_compatibleWithTraitCollection(
            &self,
            value: CGFloat,
            trait_collection: Option<&UITraitCollection>,
        ) -> CGFloat;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIFontMetrics {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

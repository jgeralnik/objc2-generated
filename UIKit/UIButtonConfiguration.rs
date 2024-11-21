//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[cfg(feature = "block2")]
pub type UIConfigurationTextAttributesTransformer = *mut block2::Block<
    dyn Fn(
        NonNull<NSDictionary<NSAttributedStringKey, AnyObject>>,
    ) -> NonNull<NSDictionary<NSAttributedStringKey, AnyObject>>,
>;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIButtonConfigurationSize(pub NSInteger);
impl UIButtonConfigurationSize {
    #[doc(alias = "UIButtonConfigurationSizeMedium")]
    pub const Medium: Self = Self(0);
    #[doc(alias = "UIButtonConfigurationSizeSmall")]
    pub const Small: Self = Self(1);
    #[doc(alias = "UIButtonConfigurationSizeMini")]
    pub const Mini: Self = Self(2);
    #[doc(alias = "UIButtonConfigurationSizeLarge")]
    pub const Large: Self = Self(3);
}

unsafe impl Encode for UIButtonConfigurationSize {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIButtonConfigurationSize {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIButtonConfigurationTitleAlignment(pub NSInteger);
impl UIButtonConfigurationTitleAlignment {
    #[doc(alias = "UIButtonConfigurationTitleAlignmentAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "UIButtonConfigurationTitleAlignmentLeading")]
    pub const Leading: Self = Self(1);
    #[doc(alias = "UIButtonConfigurationTitleAlignmentCenter")]
    pub const Center: Self = Self(2);
    #[doc(alias = "UIButtonConfigurationTitleAlignmentTrailing")]
    pub const Trailing: Self = Self(3);
}

unsafe impl Encode for UIButtonConfigurationTitleAlignment {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIButtonConfigurationTitleAlignment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIButtonConfigurationCornerStyle(pub NSInteger);
impl UIButtonConfigurationCornerStyle {
    #[doc(alias = "UIButtonConfigurationCornerStyleFixed")]
    pub const Fixed: Self = Self(-1);
    #[doc(alias = "UIButtonConfigurationCornerStyleDynamic")]
    pub const Dynamic: Self = Self(0);
    #[doc(alias = "UIButtonConfigurationCornerStyleSmall")]
    pub const Small: Self = Self(1);
    #[doc(alias = "UIButtonConfigurationCornerStyleMedium")]
    pub const Medium: Self = Self(2);
    #[doc(alias = "UIButtonConfigurationCornerStyleLarge")]
    pub const Large: Self = Self(3);
    #[doc(alias = "UIButtonConfigurationCornerStyleCapsule")]
    pub const Capsule: Self = Self(4);
}

unsafe impl Encode for UIButtonConfigurationCornerStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIButtonConfigurationCornerStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIButtonConfigurationMacIdiomStyle(pub NSInteger);
impl UIButtonConfigurationMacIdiomStyle {
    #[doc(alias = "UIButtonConfigurationMacIdiomStyleAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "UIButtonConfigurationMacIdiomStyleBordered")]
    pub const Bordered: Self = Self(1);
    #[doc(alias = "UIButtonConfigurationMacIdiomStyleBorderless")]
    pub const Borderless: Self = Self(2);
    #[doc(alias = "UIButtonConfigurationMacIdiomStyleBorderlessTinted")]
    pub const BorderlessTinted: Self = Self(3);
}

unsafe impl Encode for UIButtonConfigurationMacIdiomStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIButtonConfigurationMacIdiomStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIButtonConfigurationIndicator(pub NSInteger);
impl UIButtonConfigurationIndicator {
    #[doc(alias = "UIButtonConfigurationIndicatorAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "UIButtonConfigurationIndicatorNone")]
    pub const None: Self = Self(1);
    #[doc(alias = "UIButtonConfigurationIndicatorPopup")]
    pub const Popup: Self = Self(2);
}

unsafe impl Encode for UIButtonConfigurationIndicator {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIButtonConfigurationIndicator {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIButtonConfiguration;
);

unsafe impl NSCoding for UIButtonConfiguration {}

unsafe impl NSCopying for UIButtonConfiguration {}

unsafe impl CopyingHelper for UIButtonConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIButtonConfiguration {}

unsafe impl NSSecureCoding for UIButtonConfiguration {}

extern_methods!(
    unsafe impl UIButtonConfiguration {
        #[method_id(@__retain_semantics Other plainButtonConfiguration)]
        pub unsafe fn plainButtonConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other tintedButtonConfiguration)]
        pub unsafe fn tintedButtonConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other grayButtonConfiguration)]
        pub unsafe fn grayButtonConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other filledButtonConfiguration)]
        pub unsafe fn filledButtonConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other borderlessButtonConfiguration)]
        pub unsafe fn borderlessButtonConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other borderedButtonConfiguration)]
        pub unsafe fn borderedButtonConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other borderedTintedButtonConfiguration)]
        pub unsafe fn borderedTintedButtonConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other borderedProminentButtonConfiguration)]
        pub unsafe fn borderedProminentButtonConfiguration(mtm: MainThreadMarker)
            -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(
            feature = "UIButton",
            feature = "UIControl",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        #[method_id(@__retain_semantics Other updatedConfigurationForButton:)]
        pub unsafe fn updatedConfigurationForButton(&self, button: &UIButton) -> Retained<Self>;

        #[cfg(feature = "UIBackgroundConfiguration")]
        #[method_id(@__retain_semantics Other background)]
        pub unsafe fn background(&self) -> Retained<UIBackgroundConfiguration>;

        #[cfg(feature = "UIBackgroundConfiguration")]
        #[method(setBackground:)]
        pub unsafe fn setBackground(&self, background: &UIBackgroundConfiguration);

        #[method(cornerStyle)]
        pub unsafe fn cornerStyle(&self) -> UIButtonConfigurationCornerStyle;

        #[method(setCornerStyle:)]
        pub unsafe fn setCornerStyle(&self, corner_style: UIButtonConfigurationCornerStyle);

        #[method(buttonSize)]
        pub unsafe fn buttonSize(&self) -> UIButtonConfigurationSize;

        #[method(setButtonSize:)]
        pub unsafe fn setButtonSize(&self, button_size: UIButtonConfigurationSize);

        #[method(macIdiomStyle)]
        pub unsafe fn macIdiomStyle(&self) -> UIButtonConfigurationMacIdiomStyle;

        #[method(setMacIdiomStyle:)]
        pub unsafe fn setMacIdiomStyle(&self, mac_idiom_style: UIButtonConfigurationMacIdiomStyle);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other baseForegroundColor)]
        pub unsafe fn baseForegroundColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setBaseForegroundColor:)]
        pub unsafe fn setBaseForegroundColor(&self, base_foreground_color: Option<&UIColor>);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other baseBackgroundColor)]
        pub unsafe fn baseBackgroundColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setBaseBackgroundColor:)]
        pub unsafe fn setBaseBackgroundColor(&self, base_background_color: Option<&UIColor>);

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&UIImage>);

        #[cfg(all(
            feature = "UIColor",
            feature = "UIConfigurationColorTransformer",
            feature = "block2"
        ))]
        #[method(imageColorTransformer)]
        pub unsafe fn imageColorTransformer(&self) -> UIConfigurationColorTransformer;

        #[cfg(all(
            feature = "UIColor",
            feature = "UIConfigurationColorTransformer",
            feature = "block2"
        ))]
        #[method(setImageColorTransformer:)]
        pub unsafe fn setImageColorTransformer(
            &self,
            image_color_transformer: UIConfigurationColorTransformer,
        );

        #[cfg(all(
            feature = "UIImageConfiguration",
            feature = "UIImageSymbolConfiguration"
        ))]
        #[method_id(@__retain_semantics Other preferredSymbolConfigurationForImage)]
        pub unsafe fn preferredSymbolConfigurationForImage(
            &self,
        ) -> Option<Retained<UIImageSymbolConfiguration>>;

        #[cfg(all(
            feature = "UIImageConfiguration",
            feature = "UIImageSymbolConfiguration"
        ))]
        #[method(setPreferredSymbolConfigurationForImage:)]
        pub unsafe fn setPreferredSymbolConfigurationForImage(
            &self,
            preferred_symbol_configuration_for_image: Option<&UIImageSymbolConfiguration>,
        );

        #[method(showsActivityIndicator)]
        pub unsafe fn showsActivityIndicator(&self) -> bool;

        #[method(setShowsActivityIndicator:)]
        pub unsafe fn setShowsActivityIndicator(&self, shows_activity_indicator: bool);

        #[cfg(all(
            feature = "UIColor",
            feature = "UIConfigurationColorTransformer",
            feature = "block2"
        ))]
        #[method(activityIndicatorColorTransformer)]
        pub unsafe fn activityIndicatorColorTransformer(&self) -> UIConfigurationColorTransformer;

        #[cfg(all(
            feature = "UIColor",
            feature = "UIConfigurationColorTransformer",
            feature = "block2"
        ))]
        #[method(setActivityIndicatorColorTransformer:)]
        pub unsafe fn setActivityIndicatorColorTransformer(
            &self,
            activity_indicator_color_transformer: UIConfigurationColorTransformer,
        );

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Retained<NSString>>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[method_id(@__retain_semantics Other attributedTitle)]
        pub unsafe fn attributedTitle(&self) -> Option<Retained<NSAttributedString>>;

        #[method(setAttributedTitle:)]
        pub unsafe fn setAttributedTitle(&self, attributed_title: Option<&NSAttributedString>);

        #[cfg(feature = "block2")]
        #[method(titleTextAttributesTransformer)]
        pub unsafe fn titleTextAttributesTransformer(
            &self,
        ) -> UIConfigurationTextAttributesTransformer;

        #[cfg(feature = "block2")]
        #[method(setTitleTextAttributesTransformer:)]
        pub unsafe fn setTitleTextAttributesTransformer(
            &self,
            title_text_attributes_transformer: UIConfigurationTextAttributesTransformer,
        );

        #[cfg(feature = "NSParagraphStyle")]
        #[method(titleLineBreakMode)]
        pub unsafe fn titleLineBreakMode(&self) -> NSLineBreakMode;

        #[cfg(feature = "NSParagraphStyle")]
        #[method(setTitleLineBreakMode:)]
        pub unsafe fn setTitleLineBreakMode(&self, title_line_break_mode: NSLineBreakMode);

        #[method_id(@__retain_semantics Other subtitle)]
        pub unsafe fn subtitle(&self) -> Option<Retained<NSString>>;

        #[method(setSubtitle:)]
        pub unsafe fn setSubtitle(&self, subtitle: Option<&NSString>);

        #[method_id(@__retain_semantics Other attributedSubtitle)]
        pub unsafe fn attributedSubtitle(&self) -> Option<Retained<NSAttributedString>>;

        #[method(setAttributedSubtitle:)]
        pub unsafe fn setAttributedSubtitle(
            &self,
            attributed_subtitle: Option<&NSAttributedString>,
        );

        #[cfg(feature = "block2")]
        #[method(subtitleTextAttributesTransformer)]
        pub unsafe fn subtitleTextAttributesTransformer(
            &self,
        ) -> UIConfigurationTextAttributesTransformer;

        #[cfg(feature = "block2")]
        #[method(setSubtitleTextAttributesTransformer:)]
        pub unsafe fn setSubtitleTextAttributesTransformer(
            &self,
            subtitle_text_attributes_transformer: UIConfigurationTextAttributesTransformer,
        );

        #[cfg(feature = "NSParagraphStyle")]
        #[method(subtitleLineBreakMode)]
        pub unsafe fn subtitleLineBreakMode(&self) -> NSLineBreakMode;

        #[cfg(feature = "NSParagraphStyle")]
        #[method(setSubtitleLineBreakMode:)]
        pub unsafe fn setSubtitleLineBreakMode(&self, subtitle_line_break_mode: NSLineBreakMode);

        #[method(indicator)]
        pub unsafe fn indicator(&self) -> UIButtonConfigurationIndicator;

        #[method(setIndicator:)]
        pub unsafe fn setIndicator(&self, indicator: UIButtonConfigurationIndicator);

        #[cfg(all(
            feature = "UIColor",
            feature = "UIConfigurationColorTransformer",
            feature = "block2"
        ))]
        #[method(indicatorColorTransformer)]
        pub unsafe fn indicatorColorTransformer(&self) -> UIConfigurationColorTransformer;

        #[cfg(all(
            feature = "UIColor",
            feature = "UIConfigurationColorTransformer",
            feature = "block2"
        ))]
        #[method(setIndicatorColorTransformer:)]
        pub unsafe fn setIndicatorColorTransformer(
            &self,
            indicator_color_transformer: UIConfigurationColorTransformer,
        );

        #[cfg(feature = "UIGeometry")]
        #[method(contentInsets)]
        pub unsafe fn contentInsets(&self) -> NSDirectionalEdgeInsets;

        #[cfg(feature = "UIGeometry")]
        #[method(setContentInsets:)]
        pub unsafe fn setContentInsets(&self, content_insets: NSDirectionalEdgeInsets);

        #[method(setDefaultContentInsets)]
        pub unsafe fn setDefaultContentInsets(&self);

        #[cfg(feature = "UIGeometry")]
        #[method(imagePlacement)]
        pub unsafe fn imagePlacement(&self) -> NSDirectionalRectEdge;

        #[cfg(feature = "UIGeometry")]
        #[method(setImagePlacement:)]
        pub unsafe fn setImagePlacement(&self, image_placement: NSDirectionalRectEdge);

        #[method(imagePadding)]
        pub unsafe fn imagePadding(&self) -> CGFloat;

        #[method(setImagePadding:)]
        pub unsafe fn setImagePadding(&self, image_padding: CGFloat);

        #[method(titlePadding)]
        pub unsafe fn titlePadding(&self) -> CGFloat;

        #[method(setTitlePadding:)]
        pub unsafe fn setTitlePadding(&self, title_padding: CGFloat);

        #[method(titleAlignment)]
        pub unsafe fn titleAlignment(&self) -> UIButtonConfigurationTitleAlignment;

        #[method(setTitleAlignment:)]
        pub unsafe fn setTitleAlignment(
            &self,
            title_alignment: UIButtonConfigurationTitleAlignment,
        );

        #[method(automaticallyUpdateForSelection)]
        pub unsafe fn automaticallyUpdateForSelection(&self) -> bool;

        #[method(setAutomaticallyUpdateForSelection:)]
        pub unsafe fn setAutomaticallyUpdateForSelection(
            &self,
            automatically_update_for_selection: bool,
        );
    }
);

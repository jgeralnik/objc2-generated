//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIListContentConfiguration;
);

unsafe impl NSCoding for UIListContentConfiguration {}

unsafe impl NSCopying for UIListContentConfiguration {}

unsafe impl CopyingHelper for UIListContentConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIListContentConfiguration {}

unsafe impl NSSecureCoding for UIListContentConfiguration {}

#[cfg(feature = "UIContentConfiguration")]
unsafe impl UIContentConfiguration for UIListContentConfiguration {}

extern_methods!(
    unsafe impl UIListContentConfiguration {
        #[method_id(@__retain_semantics Other cellConfiguration)]
        pub unsafe fn cellConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other subtitleCellConfiguration)]
        pub unsafe fn subtitleCellConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other valueCellConfiguration)]
        pub unsafe fn valueCellConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other headerConfiguration)]
        pub unsafe fn headerConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other footerConfiguration)]
        pub unsafe fn footerConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other prominentInsetGroupedHeaderConfiguration)]
        pub unsafe fn prominentInsetGroupedHeaderConfiguration(
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other extraProminentInsetGroupedHeaderConfiguration)]
        pub unsafe fn extraProminentInsetGroupedHeaderConfiguration(
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other accompaniedSidebarCellConfiguration)]
        pub unsafe fn accompaniedSidebarCellConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other accompaniedSidebarSubtitleCellConfiguration)]
        pub unsafe fn accompaniedSidebarSubtitleCellConfiguration(
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&UIImage>);

        #[cfg(feature = "UIListContentImageProperties")]
        #[method_id(@__retain_semantics Other imageProperties)]
        pub unsafe fn imageProperties(&self) -> Retained<UIListContentImageProperties>;

        #[method_id(@__retain_semantics Other text)]
        pub unsafe fn text(&self) -> Option<Retained<NSString>>;

        #[method(setText:)]
        pub unsafe fn setText(&self, text: Option<&NSString>);

        #[method_id(@__retain_semantics Other attributedText)]
        pub unsafe fn attributedText(&self) -> Option<Retained<NSAttributedString>>;

        #[method(setAttributedText:)]
        pub unsafe fn setAttributedText(&self, attributed_text: Option<&NSAttributedString>);

        #[cfg(feature = "UIListContentTextProperties")]
        #[method_id(@__retain_semantics Other textProperties)]
        pub unsafe fn textProperties(&self) -> Retained<UIListContentTextProperties>;

        #[method_id(@__retain_semantics Other secondaryText)]
        pub unsafe fn secondaryText(&self) -> Option<Retained<NSString>>;

        #[method(setSecondaryText:)]
        pub unsafe fn setSecondaryText(&self, secondary_text: Option<&NSString>);

        #[method_id(@__retain_semantics Other secondaryAttributedText)]
        pub unsafe fn secondaryAttributedText(&self) -> Option<Retained<NSAttributedString>>;

        #[method(setSecondaryAttributedText:)]
        pub unsafe fn setSecondaryAttributedText(
            &self,
            secondary_attributed_text: Option<&NSAttributedString>,
        );

        #[cfg(feature = "UIListContentTextProperties")]
        #[method_id(@__retain_semantics Other secondaryTextProperties)]
        pub unsafe fn secondaryTextProperties(&self) -> Retained<UIListContentTextProperties>;

        #[cfg(feature = "UIGeometry")]
        #[method(axesPreservingSuperviewLayoutMargins)]
        pub unsafe fn axesPreservingSuperviewLayoutMargins(&self) -> UIAxis;

        #[cfg(feature = "UIGeometry")]
        #[method(setAxesPreservingSuperviewLayoutMargins:)]
        pub unsafe fn setAxesPreservingSuperviewLayoutMargins(
            &self,
            axes_preserving_superview_layout_margins: UIAxis,
        );

        #[cfg(feature = "UIGeometry")]
        #[method(directionalLayoutMargins)]
        pub unsafe fn directionalLayoutMargins(&self) -> NSDirectionalEdgeInsets;

        #[cfg(feature = "UIGeometry")]
        #[method(setDirectionalLayoutMargins:)]
        pub unsafe fn setDirectionalLayoutMargins(
            &self,
            directional_layout_margins: NSDirectionalEdgeInsets,
        );

        #[method(prefersSideBySideTextAndSecondaryText)]
        pub unsafe fn prefersSideBySideTextAndSecondaryText(&self) -> bool;

        #[method(setPrefersSideBySideTextAndSecondaryText:)]
        pub unsafe fn setPrefersSideBySideTextAndSecondaryText(
            &self,
            prefers_side_by_side_text_and_secondary_text: bool,
        );

        #[method(imageToTextPadding)]
        pub unsafe fn imageToTextPadding(&self) -> CGFloat;

        #[method(setImageToTextPadding:)]
        pub unsafe fn setImageToTextPadding(&self, image_to_text_padding: CGFloat);

        #[method(textToSecondaryTextHorizontalPadding)]
        pub unsafe fn textToSecondaryTextHorizontalPadding(&self) -> CGFloat;

        #[method(setTextToSecondaryTextHorizontalPadding:)]
        pub unsafe fn setTextToSecondaryTextHorizontalPadding(
            &self,
            text_to_secondary_text_horizontal_padding: CGFloat,
        );

        #[method(textToSecondaryTextVerticalPadding)]
        pub unsafe fn textToSecondaryTextVerticalPadding(&self) -> CGFloat;

        #[method(setTextToSecondaryTextVerticalPadding:)]
        pub unsafe fn setTextToSecondaryTextVerticalPadding(
            &self,
            text_to_secondary_text_vertical_padding: CGFloat,
        );

        #[method(alpha)]
        pub unsafe fn alpha(&self) -> CGFloat;

        #[method(setAlpha:)]
        pub unsafe fn setAlpha(&self, alpha: CGFloat);

        #[deprecated]
        #[method_id(@__retain_semantics Other sidebarCellConfiguration)]
        pub unsafe fn sidebarCellConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other sidebarSubtitleCellConfiguration)]
        pub unsafe fn sidebarSubtitleCellConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other plainHeaderConfiguration)]
        pub unsafe fn plainHeaderConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other plainFooterConfiguration)]
        pub unsafe fn plainFooterConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other groupedHeaderConfiguration)]
        pub unsafe fn groupedHeaderConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other groupedFooterConfiguration)]
        pub unsafe fn groupedFooterConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other sidebarHeaderConfiguration)]
        pub unsafe fn sidebarHeaderConfiguration(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(UIView, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    pub struct UIListContentView;
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
unsafe impl CALayerDelegate for UIListContentView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSCoding for UIListContentView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSObjectProtocol for UIListContentView {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearance for UIListContentView {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearanceContainer for UIListContentView {}

#[cfg(all(
    feature = "UIContentConfiguration",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIContentView for UIListContentView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UICoordinateSpace for UIListContentView {}

#[cfg(all(
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UIListContentView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusEnvironment for UIListContentView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItem for UIListContentView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItemContainer for UIListContentView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UIResponderStandardEditActions for UIListContentView {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UIListContentView {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIListContentView {
        #[method_id(@__retain_semantics Init initWithConfiguration:)]
        pub unsafe fn initWithConfiguration(
            this: Allocated<Self>,
            configuration: &UIListContentConfiguration,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration(&self) -> Retained<UIListContentConfiguration>;

        #[method(setConfiguration:)]
        pub unsafe fn setConfiguration(&self, configuration: &UIListContentConfiguration);

        #[cfg(feature = "UILayoutGuide")]
        #[method_id(@__retain_semantics Other textLayoutGuide)]
        pub unsafe fn textLayoutGuide(&self) -> Option<Retained<UILayoutGuide>>;

        #[cfg(feature = "UILayoutGuide")]
        #[method_id(@__retain_semantics Other secondaryTextLayoutGuide)]
        pub unsafe fn secondaryTextLayoutGuide(&self) -> Option<Retained<UILayoutGuide>>;

        #[cfg(feature = "UILayoutGuide")]
        #[method_id(@__retain_semantics Other imageLayoutGuide)]
        pub unsafe fn imageLayoutGuide(&self) -> Option<Retained<UILayoutGuide>>;
    }
);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIBackgroundConfiguration;
);

unsafe impl NSCoding for UIBackgroundConfiguration {}

unsafe impl NSCopying for UIBackgroundConfiguration {}

unsafe impl CopyingHelper for UIBackgroundConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIBackgroundConfiguration {}

unsafe impl NSSecureCoding for UIBackgroundConfiguration {}

extern_methods!(
    unsafe impl UIBackgroundConfiguration {
        #[method_id(@__retain_semantics Other clearConfiguration)]
        pub unsafe fn clearConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other listCellConfiguration)]
        pub unsafe fn listCellConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other listHeaderConfiguration)]
        pub unsafe fn listHeaderConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other listFooterConfiguration)]
        pub unsafe fn listFooterConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other listAccompaniedSidebarCellConfiguration)]
        pub unsafe fn listAccompaniedSidebarCellConfiguration(
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "UIConfigurationState")]
        #[method_id(@__retain_semantics Other updatedConfigurationForState:)]
        pub unsafe fn updatedConfigurationForState(
            &self,
            state: &ProtocolObject<dyn UIConfigurationState>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other customView)]
        pub unsafe fn customView(&self) -> Option<Retained<UIView>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method(setCustomView:)]
        pub unsafe fn setCustomView(&self, custom_view: Option<&UIView>);

        #[method(cornerRadius)]
        pub unsafe fn cornerRadius(&self) -> CGFloat;

        #[method(setCornerRadius:)]
        pub unsafe fn setCornerRadius(&self, corner_radius: CGFloat);

        #[cfg(feature = "UIGeometry")]
        #[method(backgroundInsets)]
        pub unsafe fn backgroundInsets(&self) -> NSDirectionalEdgeInsets;

        #[cfg(feature = "UIGeometry")]
        #[method(setBackgroundInsets:)]
        pub unsafe fn setBackgroundInsets(&self, background_insets: NSDirectionalEdgeInsets);

        #[cfg(feature = "UIGeometry")]
        #[method(edgesAddingLayoutMarginsToBackgroundInsets)]
        pub unsafe fn edgesAddingLayoutMarginsToBackgroundInsets(&self) -> NSDirectionalRectEdge;

        #[cfg(feature = "UIGeometry")]
        #[method(setEdgesAddingLayoutMarginsToBackgroundInsets:)]
        pub unsafe fn setEdgesAddingLayoutMarginsToBackgroundInsets(
            &self,
            edges_adding_layout_margins_to_background_insets: NSDirectionalRectEdge,
        );

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: Option<&UIColor>);

        #[cfg(all(
            feature = "UIColor",
            feature = "UIConfigurationColorTransformer",
            feature = "block2"
        ))]
        #[method(backgroundColorTransformer)]
        pub unsafe fn backgroundColorTransformer(&self) -> UIConfigurationColorTransformer;

        #[cfg(all(
            feature = "UIColor",
            feature = "UIConfigurationColorTransformer",
            feature = "block2"
        ))]
        #[method(setBackgroundColorTransformer:)]
        pub unsafe fn setBackgroundColorTransformer(
            &self,
            background_color_transformer: UIConfigurationColorTransformer,
        );

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other resolvedBackgroundColorForTintColor:)]
        pub unsafe fn resolvedBackgroundColorForTintColor(
            &self,
            tint_color: &UIColor,
        ) -> Retained<UIColor>;

        #[cfg(feature = "UIVisualEffect")]
        #[method_id(@__retain_semantics Other visualEffect)]
        pub unsafe fn visualEffect(&self) -> Option<Retained<UIVisualEffect>>;

        #[cfg(feature = "UIVisualEffect")]
        #[method(setVisualEffect:)]
        pub unsafe fn setVisualEffect(&self, visual_effect: Option<&UIVisualEffect>);

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&UIImage>);

        #[cfg(feature = "UIView")]
        #[method(imageContentMode)]
        pub unsafe fn imageContentMode(&self) -> UIViewContentMode;

        #[cfg(feature = "UIView")]
        #[method(setImageContentMode:)]
        pub unsafe fn setImageContentMode(&self, image_content_mode: UIViewContentMode);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other strokeColor)]
        pub unsafe fn strokeColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setStrokeColor:)]
        pub unsafe fn setStrokeColor(&self, stroke_color: Option<&UIColor>);

        #[cfg(all(
            feature = "UIColor",
            feature = "UIConfigurationColorTransformer",
            feature = "block2"
        ))]
        #[method(strokeColorTransformer)]
        pub unsafe fn strokeColorTransformer(&self) -> UIConfigurationColorTransformer;

        #[cfg(all(
            feature = "UIColor",
            feature = "UIConfigurationColorTransformer",
            feature = "block2"
        ))]
        #[method(setStrokeColorTransformer:)]
        pub unsafe fn setStrokeColorTransformer(
            &self,
            stroke_color_transformer: UIConfigurationColorTransformer,
        );

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other resolvedStrokeColorForTintColor:)]
        pub unsafe fn resolvedStrokeColorForTintColor(
            &self,
            tint_color: &UIColor,
        ) -> Retained<UIColor>;

        #[method(strokeWidth)]
        pub unsafe fn strokeWidth(&self) -> CGFloat;

        #[method(setStrokeWidth:)]
        pub unsafe fn setStrokeWidth(&self, stroke_width: CGFloat);

        #[method(strokeOutset)]
        pub unsafe fn strokeOutset(&self) -> CGFloat;

        #[method(setStrokeOutset:)]
        pub unsafe fn setStrokeOutset(&self, stroke_outset: CGFloat);

        #[cfg(feature = "UIShadowProperties")]
        #[method_id(@__retain_semantics Other shadowProperties)]
        pub unsafe fn shadowProperties(&self) -> Retained<UIShadowProperties>;

        #[deprecated]
        #[method_id(@__retain_semantics Other listPlainCellConfiguration)]
        pub unsafe fn listPlainCellConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other listGroupedCellConfiguration)]
        pub unsafe fn listGroupedCellConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other listSidebarCellConfiguration)]
        pub unsafe fn listSidebarCellConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[deprecated = "Use +listHeaderConfiguration or +listFooterConfiguration"]
        #[method_id(@__retain_semantics Other listPlainHeaderFooterConfiguration)]
        pub unsafe fn listPlainHeaderFooterConfiguration(mtm: MainThreadMarker) -> Retained<Self>;

        #[deprecated = "Use +listHeaderConfiguration or +listFooterConfiguration"]
        #[method_id(@__retain_semantics Other listGroupedHeaderFooterConfiguration)]
        pub unsafe fn listGroupedHeaderFooterConfiguration(mtm: MainThreadMarker)
            -> Retained<Self>;

        #[deprecated = "Use +listHeaderConfiguration or +listFooterConfiguration"]
        #[method_id(@__retain_semantics Other listSidebarHeaderConfiguration)]
        pub unsafe fn listSidebarHeaderConfiguration(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

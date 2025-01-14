//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipointerstyle?language=objc)
    #[unsafe(super(UIHoverStyle, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIHoverStyle")]
    pub struct UIPointerStyle;
);

#[cfg(feature = "UIHoverStyle")]
unsafe impl NSCopying for UIPointerStyle {}

#[cfg(feature = "UIHoverStyle")]
unsafe impl CopyingHelper for UIPointerStyle {
    type Result = Self;
}

#[cfg(feature = "UIHoverStyle")]
unsafe impl NSObjectProtocol for UIPointerStyle {}

extern_methods!(
    #[cfg(feature = "UIHoverStyle")]
    unsafe impl UIPointerStyle {
        #[cfg(feature = "UIPointerAccessory")]
        #[method_id(@__retain_semantics Other accessories)]
        pub unsafe fn accessories(&self) -> Retained<NSArray<UIPointerAccessory>>;

        #[cfg(feature = "UIPointerAccessory")]
        #[method(setAccessories:)]
        pub unsafe fn setAccessories(&self, accessories: &NSArray<UIPointerAccessory>);

        #[method_id(@__retain_semantics Other styleWithEffect:shape:)]
        pub unsafe fn styleWithEffect_shape(
            effect: &UIPointerEffect,
            shape: Option<&UIPointerShape>,
        ) -> Retained<Self>;

        #[cfg(feature = "UIGeometry")]
        #[method_id(@__retain_semantics Other styleWithShape:constrainedAxes:)]
        pub unsafe fn styleWithShape_constrainedAxes(
            shape: &UIPointerShape,
            axes: UIAxis,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other hiddenPointerStyle)]
        pub unsafe fn hiddenPointerStyle(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other systemPointerStyle)]
        pub unsafe fn systemPointerStyle(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIHoverStyle`
    #[cfg(feature = "UIHoverStyle")]
    unsafe impl UIPointerStyle {
        #[cfg(feature = "UIShape")]
        #[method_id(@__retain_semantics Other styleWithShape:)]
        pub unsafe fn styleWithShape(
            shape: Option<&UIShape>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other automaticStyle)]
        pub unsafe fn automaticStyle(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipointereffect?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPointerEffect;
);

unsafe impl NSCopying for UIPointerEffect {}

unsafe impl CopyingHelper for UIPointerEffect {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIPointerEffect {}

#[cfg(feature = "UIHoverEffect")]
unsafe impl UIHoverEffect for UIPointerEffect {}

extern_methods!(
    unsafe impl UIPointerEffect {
        #[cfg(feature = "UITargetedPreview")]
        #[method_id(@__retain_semantics Other preview)]
        pub unsafe fn preview(&self) -> Retained<UITargetedPreview>;

        #[cfg(feature = "UITargetedPreview")]
        #[method_id(@__retain_semantics Other effectWithPreview:)]
        pub unsafe fn effectWithPreview(preview: &UITargetedPreview) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipointerhighlighteffect?language=objc)
    #[unsafe(super(UIPointerEffect, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPointerHighlightEffect;
);

unsafe impl NSCopying for UIPointerHighlightEffect {}

unsafe impl CopyingHelper for UIPointerHighlightEffect {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIPointerHighlightEffect {}

#[cfg(feature = "UIHoverEffect")]
unsafe impl UIHoverEffect for UIPointerHighlightEffect {}

extern_methods!(
    unsafe impl UIPointerHighlightEffect {}
);

extern_methods!(
    /// Methods declared on superclass `UIPointerEffect`
    unsafe impl UIPointerHighlightEffect {
        #[cfg(feature = "UITargetedPreview")]
        #[method_id(@__retain_semantics Other effectWithPreview:)]
        pub unsafe fn effectWithPreview(preview: &UITargetedPreview) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipointerlifteffect?language=objc)
    #[unsafe(super(UIPointerEffect, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPointerLiftEffect;
);

unsafe impl NSCopying for UIPointerLiftEffect {}

unsafe impl CopyingHelper for UIPointerLiftEffect {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIPointerLiftEffect {}

#[cfg(feature = "UIHoverEffect")]
unsafe impl UIHoverEffect for UIPointerLiftEffect {}

extern_methods!(
    unsafe impl UIPointerLiftEffect {}
);

extern_methods!(
    /// Methods declared on superclass `UIPointerEffect`
    unsafe impl UIPointerLiftEffect {
        #[cfg(feature = "UITargetedPreview")]
        #[method_id(@__retain_semantics Other effectWithPreview:)]
        pub unsafe fn effectWithPreview(preview: &UITargetedPreview) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipointereffecttintmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIPointerEffectTintMode(pub NSInteger);
impl UIPointerEffectTintMode {
    #[doc(alias = "UIPointerEffectTintModeNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "UIPointerEffectTintModeOverlay")]
    pub const Overlay: Self = Self(1);
    #[doc(alias = "UIPointerEffectTintModeUnderlay")]
    pub const Underlay: Self = Self(2);
}

unsafe impl Encode for UIPointerEffectTintMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIPointerEffectTintMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipointerhovereffect?language=objc)
    #[unsafe(super(UIPointerEffect, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPointerHoverEffect;
);

unsafe impl NSCopying for UIPointerHoverEffect {}

unsafe impl CopyingHelper for UIPointerHoverEffect {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIPointerHoverEffect {}

#[cfg(feature = "UIHoverEffect")]
unsafe impl UIHoverEffect for UIPointerHoverEffect {}

extern_methods!(
    unsafe impl UIPointerHoverEffect {
        #[method(preferredTintMode)]
        pub unsafe fn preferredTintMode(&self) -> UIPointerEffectTintMode;

        #[method(setPreferredTintMode:)]
        pub unsafe fn setPreferredTintMode(&self, preferred_tint_mode: UIPointerEffectTintMode);

        #[method(prefersShadow)]
        pub unsafe fn prefersShadow(&self) -> bool;

        #[method(setPrefersShadow:)]
        pub unsafe fn setPrefersShadow(&self, prefers_shadow: bool);

        #[method(prefersScaledContent)]
        pub unsafe fn prefersScaledContent(&self) -> bool;

        #[method(setPrefersScaledContent:)]
        pub unsafe fn setPrefersScaledContent(&self, prefers_scaled_content: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIPointerEffect`
    unsafe impl UIPointerHoverEffect {
        #[cfg(feature = "UITargetedPreview")]
        #[method_id(@__retain_semantics Other effectWithPreview:)]
        pub unsafe fn effectWithPreview(preview: &UITargetedPreview) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipointershape?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPointerShape;
);

unsafe impl NSCopying for UIPointerShape {}

unsafe impl CopyingHelper for UIPointerShape {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIPointerShape {}

extern_methods!(
    unsafe impl UIPointerShape {
        #[cfg(feature = "UIBezierPath")]
        #[method_id(@__retain_semantics Other shapeWithPath:)]
        pub unsafe fn shapeWithPath(path: &UIBezierPath, mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other shapeWithRoundedRect:)]
        pub unsafe fn shapeWithRoundedRect(rect: CGRect, mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other shapeWithRoundedRect:cornerRadius:)]
        pub unsafe fn shapeWithRoundedRect_cornerRadius(
            rect: CGRect,
            corner_radius: CGFloat,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "UIGeometry")]
        #[method_id(@__retain_semantics Other beamWithPreferredLength:axis:)]
        pub unsafe fn beamWithPreferredLength_axis(
            length: CGFloat,
            axis: UIAxis,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

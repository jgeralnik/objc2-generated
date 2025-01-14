//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsvisualeffectmaterial?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSVisualEffectMaterial(pub NSInteger);
impl NSVisualEffectMaterial {
    #[doc(alias = "NSVisualEffectMaterialTitlebar")]
    pub const Titlebar: Self = Self(3);
    #[doc(alias = "NSVisualEffectMaterialSelection")]
    pub const Selection: Self = Self(4);
    #[doc(alias = "NSVisualEffectMaterialMenu")]
    pub const Menu: Self = Self(5);
    #[doc(alias = "NSVisualEffectMaterialPopover")]
    pub const Popover: Self = Self(6);
    #[doc(alias = "NSVisualEffectMaterialSidebar")]
    pub const Sidebar: Self = Self(7);
    #[doc(alias = "NSVisualEffectMaterialHeaderView")]
    pub const HeaderView: Self = Self(10);
    #[doc(alias = "NSVisualEffectMaterialSheet")]
    pub const Sheet: Self = Self(11);
    #[doc(alias = "NSVisualEffectMaterialWindowBackground")]
    pub const WindowBackground: Self = Self(12);
    #[doc(alias = "NSVisualEffectMaterialHUDWindow")]
    pub const HUDWindow: Self = Self(13);
    #[doc(alias = "NSVisualEffectMaterialFullScreenUI")]
    pub const FullScreenUI: Self = Self(15);
    #[doc(alias = "NSVisualEffectMaterialToolTip")]
    pub const ToolTip: Self = Self(17);
    #[doc(alias = "NSVisualEffectMaterialContentBackground")]
    pub const ContentBackground: Self = Self(18);
    #[doc(alias = "NSVisualEffectMaterialUnderWindowBackground")]
    pub const UnderWindowBackground: Self = Self(21);
    #[doc(alias = "NSVisualEffectMaterialUnderPageBackground")]
    pub const UnderPageBackground: Self = Self(22);
    #[deprecated = "Use a specific semantic material instead."]
    #[doc(alias = "NSVisualEffectMaterialAppearanceBased")]
    pub const AppearanceBased: Self = Self(0);
    #[deprecated = "Use a semantic material instead.  To force the appearance of a view hierarchy, set the `appearance` property to an appropriate NSAppearance value."]
    #[doc(alias = "NSVisualEffectMaterialLight")]
    pub const Light: Self = Self(1);
    #[deprecated = "Use a semantic material instead.  To force the appearance of a view hierarchy, set the `appearance` property to an appropriate NSAppearance value."]
    #[doc(alias = "NSVisualEffectMaterialDark")]
    pub const Dark: Self = Self(2);
    #[deprecated = "Use a semantic material instead.  To force the appearance of a view hierarchy, set the `appearance` property to an appropriate NSAppearance value."]
    #[doc(alias = "NSVisualEffectMaterialMediumLight")]
    pub const MediumLight: Self = Self(8);
    #[deprecated = "Use a semantic material instead.  To force the appearance of a view hierarchy, set the `appearance` property to an appropriate NSAppearance value."]
    #[doc(alias = "NSVisualEffectMaterialUltraDark")]
    pub const UltraDark: Self = Self(9);
}

unsafe impl Encode for NSVisualEffectMaterial {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSVisualEffectMaterial {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsvisualeffectblendingmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSVisualEffectBlendingMode(pub NSInteger);
impl NSVisualEffectBlendingMode {
    #[doc(alias = "NSVisualEffectBlendingModeBehindWindow")]
    pub const BehindWindow: Self = Self(0);
    #[doc(alias = "NSVisualEffectBlendingModeWithinWindow")]
    pub const WithinWindow: Self = Self(1);
}

unsafe impl Encode for NSVisualEffectBlendingMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSVisualEffectBlendingMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsvisualeffectstate?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSVisualEffectState(pub NSInteger);
impl NSVisualEffectState {
    #[doc(alias = "NSVisualEffectStateFollowsWindowActiveState")]
    pub const FollowsWindowActiveState: Self = Self(0);
    #[doc(alias = "NSVisualEffectStateActive")]
    pub const Active: Self = Self(1);
    #[doc(alias = "NSVisualEffectStateInactive")]
    pub const Inactive: Self = Self(2);
}

unsafe impl Encode for NSVisualEffectState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSVisualEffectState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsvisualeffectview?language=objc)
    #[unsafe(super(NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    pub struct NSVisualEffectView;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSVisualEffectView {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSVisualEffectView {}

#[cfg(all(feature = "NSAnimation", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAnimatablePropertyContainer for NSVisualEffectView {}

#[cfg(all(feature = "NSAppearance", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAppearanceCustomization for NSVisualEffectView {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSCoding for NSVisualEffectView {}

#[cfg(all(feature = "NSDragging", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSDraggingDestination for NSVisualEffectView {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSObjectProtocol for NSVisualEffectView {}

#[cfg(all(
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSVisualEffectView {}

extern_methods!(
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSVisualEffectView {
        #[method(material)]
        pub unsafe fn material(&self) -> NSVisualEffectMaterial;

        #[method(setMaterial:)]
        pub unsafe fn setMaterial(&self, material: NSVisualEffectMaterial);

        #[cfg(feature = "NSCell")]
        #[method(interiorBackgroundStyle)]
        pub unsafe fn interiorBackgroundStyle(&self) -> NSBackgroundStyle;

        #[method(blendingMode)]
        pub unsafe fn blendingMode(&self) -> NSVisualEffectBlendingMode;

        #[method(setBlendingMode:)]
        pub unsafe fn setBlendingMode(&self, blending_mode: NSVisualEffectBlendingMode);

        #[method(state)]
        pub unsafe fn state(&self) -> NSVisualEffectState;

        #[method(setState:)]
        pub unsafe fn setState(&self, state: NSVisualEffectState);

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other maskImage)]
        pub unsafe fn maskImage(&self) -> Option<Retained<NSImage>>;

        #[cfg(feature = "NSImage")]
        #[method(setMaskImage:)]
        pub unsafe fn setMaskImage(&self, mask_image: Option<&NSImage>);

        #[method(isEmphasized)]
        pub unsafe fn isEmphasized(&self) -> bool;

        #[method(setEmphasized:)]
        pub unsafe fn setEmphasized(&self, emphasized: bool);

        #[method(viewDidMoveToWindow)]
        pub unsafe fn viewDidMoveToWindow(&self);

        #[cfg(feature = "NSWindow")]
        #[method(viewWillMoveToWindow:)]
        pub unsafe fn viewWillMoveToWindow(&self, new_window: Option<&NSWindow>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSVisualEffectView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSVisualEffectView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSVisualEffectView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

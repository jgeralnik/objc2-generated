//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSUsableScrollerParts(pub NSUInteger);
impl NSUsableScrollerParts {
    pub const NSNoScrollerParts: Self = Self(0);
    #[deprecated = "Scroller arrows are not used anymore."]
    pub const NSOnlyScrollerArrows: Self = Self(1);
    pub const NSAllScrollerParts: Self = Self(2);
}

unsafe impl Encode for NSUsableScrollerParts {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSUsableScrollerParts {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSScrollerPart(pub NSUInteger);
impl NSScrollerPart {
    pub const NSScrollerNoPart: Self = Self(0);
    pub const NSScrollerDecrementPage: Self = Self(1);
    pub const NSScrollerKnob: Self = Self(2);
    pub const NSScrollerIncrementPage: Self = Self(3);
    #[deprecated = "Scroller arrows are not used anymore."]
    pub const NSScrollerDecrementLine: Self = Self(4);
    #[deprecated = "Scroller arrows are not used anymore."]
    pub const NSScrollerIncrementLine: Self = Self(5);
    pub const NSScrollerKnobSlot: Self = Self(6);
}

unsafe impl Encode for NSScrollerPart {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSScrollerPart {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSScrollerStyle(pub NSInteger);
impl NSScrollerStyle {
    #[doc(alias = "NSScrollerStyleLegacy")]
    pub const Legacy: Self = Self(0);
    #[doc(alias = "NSScrollerStyleOverlay")]
    pub const Overlay: Self = Self(1);
}

unsafe impl Encode for NSScrollerStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSScrollerStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSScrollerKnobStyle(pub NSInteger);
impl NSScrollerKnobStyle {
    #[doc(alias = "NSScrollerKnobStyleDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "NSScrollerKnobStyleDark")]
    pub const Dark: Self = Self(1);
    #[doc(alias = "NSScrollerKnobStyleLight")]
    pub const Light: Self = Self(2);
}

unsafe impl Encode for NSScrollerKnobStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSScrollerKnobStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(NSControl, NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    pub struct NSScroller;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSScroller {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSScroller {}

#[cfg(all(
    feature = "NSAnimation",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAnimatablePropertyContainer for NSScroller {}

#[cfg(all(
    feature = "NSAppearance",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAppearanceCustomization for NSScroller {}

#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSCoding for NSScroller {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSDragging",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSDraggingDestination for NSScroller {}

#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSObjectProtocol for NSScroller {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSScroller {}

extern_methods!(
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSScroller {
        #[method(isCompatibleWithOverlayScrollers)]
        pub unsafe fn isCompatibleWithOverlayScrollers(mtm: MainThreadMarker) -> bool;

        #[cfg(feature = "NSCell")]
        #[method(scrollerWidthForControlSize:scrollerStyle:)]
        pub unsafe fn scrollerWidthForControlSize_scrollerStyle(
            control_size: NSControlSize,
            scroller_style: NSScrollerStyle,
            mtm: MainThreadMarker,
        ) -> CGFloat;

        #[method(preferredScrollerStyle)]
        pub unsafe fn preferredScrollerStyle(mtm: MainThreadMarker) -> NSScrollerStyle;

        #[method(scrollerStyle)]
        pub unsafe fn scrollerStyle(&self) -> NSScrollerStyle;

        #[method(setScrollerStyle:)]
        pub unsafe fn setScrollerStyle(&self, scroller_style: NSScrollerStyle);

        #[method(knobStyle)]
        pub unsafe fn knobStyle(&self) -> NSScrollerKnobStyle;

        #[method(setKnobStyle:)]
        pub unsafe fn setKnobStyle(&self, knob_style: NSScrollerKnobStyle);

        #[method(rectForPart:)]
        pub unsafe fn rectForPart(&self, part_code: NSScrollerPart) -> NSRect;

        #[method(checkSpaceForParts)]
        pub unsafe fn checkSpaceForParts(&self);

        #[method(usableParts)]
        pub unsafe fn usableParts(&self) -> NSUsableScrollerParts;

        #[cfg(feature = "NSCell")]
        #[method(controlSize)]
        pub unsafe fn controlSize(&self) -> NSControlSize;

        #[cfg(feature = "NSCell")]
        #[method(setControlSize:)]
        pub unsafe fn setControlSize(&self, control_size: NSControlSize);

        #[method(drawKnob)]
        pub unsafe fn drawKnob(&self);

        #[method(drawKnobSlotInRect:highlight:)]
        pub unsafe fn drawKnobSlotInRect_highlight(&self, slot_rect: NSRect, flag: bool);

        #[method(testPart:)]
        pub unsafe fn testPart(&self, point: NSPoint) -> NSScrollerPart;

        #[cfg(feature = "NSEvent")]
        #[method(trackKnob:)]
        pub unsafe fn trackKnob(&self, event: &NSEvent);

        #[method(hitPart)]
        pub unsafe fn hitPart(&self) -> NSScrollerPart;

        #[method(knobProportion)]
        pub unsafe fn knobProportion(&self) -> CGFloat;

        #[method(setKnobProportion:)]
        pub unsafe fn setKnobProportion(&self, knob_proportion: CGFloat);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSScroller {
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
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSScroller {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSScroller {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern "C" {
    pub static NSPreferredScrollerStyleDidChangeNotification: &'static NSNotificationName;
}

// NS_ENUM
#[deprecated = "Scroller arrows are not used anymore."]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSScrollArrowPosition(pub NSUInteger);
impl NSScrollArrowPosition {
    pub const NSScrollerArrowsMaxEnd: Self = Self(0);
    pub const NSScrollerArrowsMinEnd: Self = Self(1);
    pub const NSScrollerArrowsDefaultSetting: Self = Self(0);
    pub const NSScrollerArrowsNone: Self = Self(2);
}

unsafe impl Encode for NSScrollArrowPosition {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSScrollArrowPosition {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[deprecated = "Scroller arrows are not used anymore."]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSScrollerArrow(pub NSUInteger);
impl NSScrollerArrow {
    pub const NSScrollerIncrementArrow: Self = Self(0);
    pub const NSScrollerDecrementArrow: Self = Self(1);
}

unsafe impl Encode for NSScrollerArrow {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSScrollerArrow {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// NSDeprecated
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSScroller {
        #[cfg(feature = "NSCell")]
        #[deprecated = "Use +scrollerWidthForControlSize:scrollerStyle: instead"]
        #[method(scrollerWidthForControlSize:)]
        pub unsafe fn scrollerWidthForControlSize(
            control_size: NSControlSize,
            mtm: MainThreadMarker,
        ) -> CGFloat;

        #[deprecated = "Use +scrollerWidthForControlSize:scrollerStyle: instead"]
        #[method(scrollerWidth)]
        pub unsafe fn scrollerWidth(mtm: MainThreadMarker) -> CGFloat;

        #[deprecated]
        #[method(setFloatValue:knobProportion:)]
        pub unsafe fn setFloatValue_knobProportion(&self, value: c_float, proportion: CGFloat);

        #[deprecated = "Has had no effect since 10.7"]
        #[method(arrowsPosition)]
        pub unsafe fn arrowsPosition(&self) -> NSScrollArrowPosition;

        #[deprecated = "Has had no effect since 10.7"]
        #[method(setArrowsPosition:)]
        pub unsafe fn setArrowsPosition(&self, arrows_position: NSScrollArrowPosition);

        #[cfg(feature = "NSCell")]
        #[deprecated = "Has had no effect since 10.7"]
        #[method(controlTint)]
        pub unsafe fn controlTint(&self) -> NSControlTint;

        #[cfg(feature = "NSCell")]
        #[deprecated = "Has had no effect since 10.7"]
        #[method(setControlTint:)]
        pub unsafe fn setControlTint(&self, control_tint: NSControlTint);

        #[deprecated = "Has had no effect since 10.7"]
        #[method(highlight:)]
        pub unsafe fn highlight(&self, flag: bool);

        #[cfg(feature = "NSEvent")]
        #[deprecated = "Not invoked since 10.7"]
        #[method(trackScrollButtons:)]
        pub unsafe fn trackScrollButtons(&self, event: &NSEvent);

        #[deprecated = "Not invoked on any macOS version"]
        #[method(drawParts)]
        pub unsafe fn drawParts(&self);

        #[deprecated = "Scrollers don't have arrows as of 10.7"]
        #[method(drawArrow:highlight:)]
        pub unsafe fn drawArrow_highlight(&self, which_arrow: NSScrollerArrow, flag: bool);
    }
);

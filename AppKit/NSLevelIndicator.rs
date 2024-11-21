//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSLevelIndicatorPlaceholderVisibility(pub NSInteger);
impl NSLevelIndicatorPlaceholderVisibility {
    #[doc(alias = "NSLevelIndicatorPlaceholderVisibilityAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "NSLevelIndicatorPlaceholderVisibilityAlways")]
    pub const Always: Self = Self(1);
    #[doc(alias = "NSLevelIndicatorPlaceholderVisibilityWhileEditing")]
    pub const WhileEditing: Self = Self(2);
}

unsafe impl Encode for NSLevelIndicatorPlaceholderVisibility {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSLevelIndicatorPlaceholderVisibility {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(NSControl, NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    pub struct NSLevelIndicator;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSLevelIndicator {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSLevelIndicator {}

#[cfg(all(
    feature = "NSAnimation",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAnimatablePropertyContainer for NSLevelIndicator {}

#[cfg(all(
    feature = "NSAppearance",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAppearanceCustomization for NSLevelIndicator {}

#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSCoding for NSLevelIndicator {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSDragging",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSDraggingDestination for NSLevelIndicator {}

#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSObjectProtocol for NSLevelIndicator {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSLevelIndicator {}

extern_methods!(
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSLevelIndicator {
        #[cfg(feature = "NSLevelIndicatorCell")]
        #[method(levelIndicatorStyle)]
        pub unsafe fn levelIndicatorStyle(&self) -> NSLevelIndicatorStyle;

        #[cfg(feature = "NSLevelIndicatorCell")]
        #[method(setLevelIndicatorStyle:)]
        pub unsafe fn setLevelIndicatorStyle(&self, level_indicator_style: NSLevelIndicatorStyle);

        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[method(minValue)]
        pub unsafe fn minValue(&self) -> c_double;

        #[method(setMinValue:)]
        pub unsafe fn setMinValue(&self, min_value: c_double);

        #[method(maxValue)]
        pub unsafe fn maxValue(&self) -> c_double;

        #[method(setMaxValue:)]
        pub unsafe fn setMaxValue(&self, max_value: c_double);

        #[method(warningValue)]
        pub unsafe fn warningValue(&self) -> c_double;

        #[method(setWarningValue:)]
        pub unsafe fn setWarningValue(&self, warning_value: c_double);

        #[method(criticalValue)]
        pub unsafe fn criticalValue(&self) -> c_double;

        #[method(setCriticalValue:)]
        pub unsafe fn setCriticalValue(&self, critical_value: c_double);

        #[cfg(feature = "NSSliderCell")]
        #[method(tickMarkPosition)]
        pub unsafe fn tickMarkPosition(&self) -> NSTickMarkPosition;

        #[cfg(feature = "NSSliderCell")]
        #[method(setTickMarkPosition:)]
        pub unsafe fn setTickMarkPosition(&self, tick_mark_position: NSTickMarkPosition);

        #[method(numberOfTickMarks)]
        pub unsafe fn numberOfTickMarks(&self) -> NSInteger;

        #[method(setNumberOfTickMarks:)]
        pub unsafe fn setNumberOfTickMarks(&self, number_of_tick_marks: NSInteger);

        #[method(numberOfMajorTickMarks)]
        pub unsafe fn numberOfMajorTickMarks(&self) -> NSInteger;

        #[method(setNumberOfMajorTickMarks:)]
        pub unsafe fn setNumberOfMajorTickMarks(&self, number_of_major_tick_marks: NSInteger);

        #[method(tickMarkValueAtIndex:)]
        pub unsafe fn tickMarkValueAtIndex(&self, index: NSInteger) -> c_double;

        #[method(rectOfTickMarkAtIndex:)]
        pub unsafe fn rectOfTickMarkAtIndex(&self, index: NSInteger) -> NSRect;

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other fillColor)]
        pub unsafe fn fillColor(&self) -> Retained<NSColor>;

        #[cfg(feature = "NSColor")]
        #[method(setFillColor:)]
        pub unsafe fn setFillColor(&self, fill_color: Option<&NSColor>);

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other warningFillColor)]
        pub unsafe fn warningFillColor(&self) -> Retained<NSColor>;

        #[cfg(feature = "NSColor")]
        #[method(setWarningFillColor:)]
        pub unsafe fn setWarningFillColor(&self, warning_fill_color: Option<&NSColor>);

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other criticalFillColor)]
        pub unsafe fn criticalFillColor(&self) -> Retained<NSColor>;

        #[cfg(feature = "NSColor")]
        #[method(setCriticalFillColor:)]
        pub unsafe fn setCriticalFillColor(&self, critical_fill_color: Option<&NSColor>);

        #[method(drawsTieredCapacityLevels)]
        pub unsafe fn drawsTieredCapacityLevels(&self) -> bool;

        #[method(setDrawsTieredCapacityLevels:)]
        pub unsafe fn setDrawsTieredCapacityLevels(&self, draws_tiered_capacity_levels: bool);

        #[method(placeholderVisibility)]
        pub unsafe fn placeholderVisibility(&self) -> NSLevelIndicatorPlaceholderVisibility;

        #[method(setPlaceholderVisibility:)]
        pub unsafe fn setPlaceholderVisibility(
            &self,
            placeholder_visibility: NSLevelIndicatorPlaceholderVisibility,
        );

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other ratingImage)]
        pub unsafe fn ratingImage(&self) -> Option<Retained<NSImage>>;

        #[cfg(feature = "NSImage")]
        #[method(setRatingImage:)]
        pub unsafe fn setRatingImage(&self, rating_image: Option<&NSImage>);

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other ratingPlaceholderImage)]
        pub unsafe fn ratingPlaceholderImage(&self) -> Option<Retained<NSImage>>;

        #[cfg(feature = "NSImage")]
        #[method(setRatingPlaceholderImage:)]
        pub unsafe fn setRatingPlaceholderImage(&self, rating_placeholder_image: Option<&NSImage>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSLevelIndicator {
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
    unsafe impl NSLevelIndicator {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSLevelIndicator {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

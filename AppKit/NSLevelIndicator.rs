//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSLevelIndicatorPlaceholderVisibility {
        #[doc(alias = "NSLevelIndicatorPlaceholderVisibilityAutomatic")]
        Automatic = 0,
        #[doc(alias = "NSLevelIndicatorPlaceholderVisibilityAlways")]
        Always = 1,
        #[doc(alias = "NSLevelIndicatorPlaceholderVisibilityWhileEditing")]
        WhileEditing = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSLevelIndicator")]
    pub struct NSLevelIndicator;

    #[cfg(feature = "AppKit_NSLevelIndicator")]
    unsafe impl ClassType for NSLevelIndicator {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = NSControl;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSLevelIndicator")]
unsafe impl NSAccessibility for NSLevelIndicator {}

#[cfg(feature = "AppKit_NSLevelIndicator")]
unsafe impl NSAccessibilityElementProtocol for NSLevelIndicator {}

#[cfg(feature = "AppKit_NSLevelIndicator")]
unsafe impl NSAnimatablePropertyContainer for NSLevelIndicator {}

#[cfg(feature = "AppKit_NSLevelIndicator")]
unsafe impl NSAppearanceCustomization for NSLevelIndicator {}

#[cfg(feature = "AppKit_NSLevelIndicator")]
unsafe impl NSCoding for NSLevelIndicator {}

#[cfg(feature = "AppKit_NSLevelIndicator")]
unsafe impl NSDraggingDestination for NSLevelIndicator {}

#[cfg(feature = "AppKit_NSLevelIndicator")]
unsafe impl NSObjectProtocol for NSLevelIndicator {}

#[cfg(feature = "AppKit_NSLevelIndicator")]
unsafe impl NSUserInterfaceItemIdentification for NSLevelIndicator {}

extern_methods!(
    #[cfg(feature = "AppKit_NSLevelIndicator")]
    unsafe impl NSLevelIndicator {
        #[method(levelIndicatorStyle)]
        pub unsafe fn levelIndicatorStyle(&self) -> NSLevelIndicatorStyle;

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

        #[method(tickMarkPosition)]
        pub unsafe fn tickMarkPosition(&self) -> NSTickMarkPosition;

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

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other fillColor)]
        pub unsafe fn fillColor(&self) -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setFillColor:)]
        pub unsafe fn setFillColor(&self, fill_color: Option<&NSColor>);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other warningFillColor)]
        pub unsafe fn warningFillColor(&self) -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setWarningFillColor:)]
        pub unsafe fn setWarningFillColor(&self, warning_fill_color: Option<&NSColor>);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other criticalFillColor)]
        pub unsafe fn criticalFillColor(&self) -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSColor")]
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

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other ratingImage)]
        pub unsafe fn ratingImage(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setRatingImage:)]
        pub unsafe fn setRatingImage(&self, rating_image: Option<&NSImage>);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other ratingPlaceholderImage)]
        pub unsafe fn ratingPlaceholderImage(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setRatingPlaceholderImage:)]
        pub unsafe fn setRatingPlaceholderImage(&self, rating_placeholder_image: Option<&NSImage>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSLevelIndicator")]
    unsafe impl NSLevelIndicator {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "AppKit_NSLevelIndicator")]
    unsafe impl NSLevelIndicator {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSLevelIndicator")]
    unsafe impl NSLevelIndicator {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

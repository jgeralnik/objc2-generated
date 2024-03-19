//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSLevelIndicatorStyle {
        #[doc(alias = "NSLevelIndicatorStyleRelevancy")]
        Relevancy = 0,
        #[doc(alias = "NSLevelIndicatorStyleContinuousCapacity")]
        ContinuousCapacity = 1,
        #[doc(alias = "NSLevelIndicatorStyleDiscreteCapacity")]
        DiscreteCapacity = 2,
        #[doc(alias = "NSLevelIndicatorStyleRating")]
        Rating = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "AppKit_NSActionCell", feature = "AppKit_NSCell"))]
    pub struct NSLevelIndicatorCell;

    #[cfg(all(feature = "AppKit_NSActionCell", feature = "AppKit_NSCell"))]
    unsafe impl ClassType for NSLevelIndicatorCell {
        #[inherits(NSCell, NSObject)]
        type Super = NSActionCell;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSActionCell",
    feature = "AppKit_NSCell"
))]
unsafe impl NSAccessibility for NSLevelIndicatorCell {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSActionCell",
    feature = "AppKit_NSCell"
))]
unsafe impl NSAccessibilityElementProtocol for NSLevelIndicatorCell {}

#[cfg(all(
    feature = "AppKit_NSActionCell",
    feature = "AppKit_NSCell",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCoding for NSLevelIndicatorCell {}

#[cfg(all(
    feature = "AppKit_NSActionCell",
    feature = "AppKit_NSCell",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCopying for NSLevelIndicatorCell {}

#[cfg(all(feature = "AppKit_NSActionCell", feature = "AppKit_NSCell"))]
unsafe impl NSObjectProtocol for NSLevelIndicatorCell {}

#[cfg(all(
    feature = "AppKit_NSActionCell",
    feature = "AppKit_NSCell",
    feature = "AppKit_NSUserInterfaceItemIdentification"
))]
unsafe impl NSUserInterfaceItemIdentification for NSLevelIndicatorCell {}

extern_methods!(
    #[cfg(all(feature = "AppKit_NSActionCell", feature = "AppKit_NSCell"))]
    unsafe impl NSLevelIndicatorCell {
        #[method_id(@__retain_semantics Init initWithLevelIndicatorStyle:)]
        pub unsafe fn initWithLevelIndicatorStyle(
            this: Allocated<Self>,
            level_indicator_style: NSLevelIndicatorStyle,
        ) -> Id<Self>;

        #[method(levelIndicatorStyle)]
        pub unsafe fn levelIndicatorStyle(&self) -> NSLevelIndicatorStyle;

        #[method(setLevelIndicatorStyle:)]
        pub unsafe fn setLevelIndicatorStyle(&self, level_indicator_style: NSLevelIndicatorStyle);

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

        #[cfg(feature = "AppKit_NSSliderCell")]
        #[method(tickMarkPosition)]
        pub unsafe fn tickMarkPosition(&self) -> NSTickMarkPosition;

        #[cfg(feature = "AppKit_NSSliderCell")]
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

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(rectOfTickMarkAtIndex:)]
        pub unsafe fn rectOfTickMarkAtIndex(&self, index: NSInteger) -> NSRect;

        #[method(tickMarkValueAtIndex:)]
        pub unsafe fn tickMarkValueAtIndex(&self, index: NSInteger) -> c_double;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(all(feature = "AppKit_NSActionCell", feature = "AppKit_NSCell"))]
    unsafe impl NSLevelIndicatorCell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(this: Allocated<Self>, string: &NSString) -> Id<Self>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(this: Allocated<Self>, image: Option<&NSImage>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "AppKit_NSActionCell", feature = "AppKit_NSCell"))]
    unsafe impl NSLevelIndicatorCell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

pub static NSRelevancyLevelIndicatorStyle: NSLevelIndicatorStyle =
    NSLevelIndicatorStyle(NSLevelIndicatorStyle::Relevancy.0);

pub static NSContinuousCapacityLevelIndicatorStyle: NSLevelIndicatorStyle =
    NSLevelIndicatorStyle(NSLevelIndicatorStyle::ContinuousCapacity.0);

pub static NSDiscreteCapacityLevelIndicatorStyle: NSLevelIndicatorStyle =
    NSLevelIndicatorStyle(NSLevelIndicatorStyle::DiscreteCapacity.0);

pub static NSRatingLevelIndicatorStyle: NSLevelIndicatorStyle =
    NSLevelIndicatorStyle(NSLevelIndicatorStyle::Rating.0);

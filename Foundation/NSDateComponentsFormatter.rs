//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSDateComponentsFormatterUnitsStyle {
        #[doc(alias = "NSDateComponentsFormatterUnitsStylePositional")]
        Positional = 0,
        #[doc(alias = "NSDateComponentsFormatterUnitsStyleAbbreviated")]
        Abbreviated = 1,
        #[doc(alias = "NSDateComponentsFormatterUnitsStyleShort")]
        Short = 2,
        #[doc(alias = "NSDateComponentsFormatterUnitsStyleFull")]
        Full = 3,
        #[doc(alias = "NSDateComponentsFormatterUnitsStyleSpellOut")]
        SpellOut = 4,
        #[doc(alias = "NSDateComponentsFormatterUnitsStyleBrief")]
        Brief = 5,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSDateComponentsFormatterZeroFormattingBehavior {
        #[doc(alias = "NSDateComponentsFormatterZeroFormattingBehaviorNone")]
        None = 0,
        #[doc(alias = "NSDateComponentsFormatterZeroFormattingBehaviorDefault")]
        Default = 1 << 0,
        #[doc(alias = "NSDateComponentsFormatterZeroFormattingBehaviorDropLeading")]
        DropLeading = 1 << 1,
        #[doc(alias = "NSDateComponentsFormatterZeroFormattingBehaviorDropMiddle")]
        DropMiddle = 1 << 2,
        #[doc(alias = "NSDateComponentsFormatterZeroFormattingBehaviorDropTrailing")]
        DropTrailing = 1 << 3,
        #[doc(alias = "NSDateComponentsFormatterZeroFormattingBehaviorDropAll")]
        DropAll = NSDateComponentsFormatterZeroFormattingBehavior::DropLeading.0
            | NSDateComponentsFormatterZeroFormattingBehavior::DropMiddle.0
            | NSDateComponentsFormatterZeroFormattingBehavior::DropTrailing.0,
        #[doc(alias = "NSDateComponentsFormatterZeroFormattingBehaviorPad")]
        Pad = 1 << 16,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSDateComponentsFormatter")]
    pub struct NSDateComponentsFormatter;

    #[cfg(feature = "Foundation_NSDateComponentsFormatter")]
    unsafe impl ClassType for NSDateComponentsFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSDateComponentsFormatter")]
unsafe impl Send for NSDateComponentsFormatter {}

#[cfg(feature = "Foundation_NSDateComponentsFormatter")]
unsafe impl Sync for NSDateComponentsFormatter {}

#[cfg(feature = "Foundation_NSDateComponentsFormatter")]
unsafe impl NSCoding for NSDateComponentsFormatter {}

#[cfg(feature = "Foundation_NSDateComponentsFormatter")]
unsafe impl NSCopying for NSDateComponentsFormatter {}

#[cfg(feature = "Foundation_NSDateComponentsFormatter")]
unsafe impl NSObjectProtocol for NSDateComponentsFormatter {}

extern_methods!(
    #[cfg(feature = "Foundation_NSDateComponentsFormatter")]
    unsafe impl NSDateComponentsFormatter {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringForObjectValue:)]
        pub unsafe fn stringForObjectValue(&self, obj: Option<&AnyObject>) -> Option<Id<NSString>>;

        #[cfg(all(
            feature = "Foundation_NSDateComponents",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other stringFromDateComponents:)]
        pub unsafe fn stringFromDateComponents(
            &self,
            components: &NSDateComponents,
        ) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSDate", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other stringFromDate:toDate:)]
        pub unsafe fn stringFromDate_toDate(
            &self,
            start_date: &NSDate,
            end_date: &NSDate,
        ) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringFromTimeInterval:)]
        pub unsafe fn stringFromTimeInterval(&self, ti: NSTimeInterval) -> Option<Id<NSString>>;

        #[cfg(all(
            feature = "Foundation_NSDateComponents",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other localizedStringFromDateComponents:unitsStyle:)]
        pub unsafe fn localizedStringFromDateComponents_unitsStyle(
            components: &NSDateComponents,
            units_style: NSDateComponentsFormatterUnitsStyle,
        ) -> Option<Id<NSString>>;

        #[method(unitsStyle)]
        pub unsafe fn unitsStyle(&self) -> NSDateComponentsFormatterUnitsStyle;

        #[method(setUnitsStyle:)]
        pub unsafe fn setUnitsStyle(&self, units_style: NSDateComponentsFormatterUnitsStyle);

        #[method(allowedUnits)]
        pub unsafe fn allowedUnits(&self) -> NSCalendarUnit;

        #[method(setAllowedUnits:)]
        pub unsafe fn setAllowedUnits(&self, allowed_units: NSCalendarUnit);

        #[method(zeroFormattingBehavior)]
        pub unsafe fn zeroFormattingBehavior(
            &self,
        ) -> NSDateComponentsFormatterZeroFormattingBehavior;

        #[method(setZeroFormattingBehavior:)]
        pub unsafe fn setZeroFormattingBehavior(
            &self,
            zero_formatting_behavior: NSDateComponentsFormatterZeroFormattingBehavior,
        );

        #[cfg(feature = "Foundation_NSCalendar")]
        #[method_id(@__retain_semantics Other calendar)]
        pub unsafe fn calendar(&self) -> Option<Id<NSCalendar>>;

        #[cfg(feature = "Foundation_NSCalendar")]
        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>);

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other referenceDate)]
        pub unsafe fn referenceDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(setReferenceDate:)]
        pub unsafe fn setReferenceDate(&self, reference_date: Option<&NSDate>);

        #[method(allowsFractionalUnits)]
        pub unsafe fn allowsFractionalUnits(&self) -> bool;

        #[method(setAllowsFractionalUnits:)]
        pub unsafe fn setAllowsFractionalUnits(&self, allows_fractional_units: bool);

        #[method(maximumUnitCount)]
        pub unsafe fn maximumUnitCount(&self) -> NSInteger;

        #[method(setMaximumUnitCount:)]
        pub unsafe fn setMaximumUnitCount(&self, maximum_unit_count: NSInteger);

        #[method(collapsesLargestUnit)]
        pub unsafe fn collapsesLargestUnit(&self) -> bool;

        #[method(setCollapsesLargestUnit:)]
        pub unsafe fn setCollapsesLargestUnit(&self, collapses_largest_unit: bool);

        #[method(includesApproximationPhrase)]
        pub unsafe fn includesApproximationPhrase(&self) -> bool;

        #[method(setIncludesApproximationPhrase:)]
        pub unsafe fn setIncludesApproximationPhrase(&self, includes_approximation_phrase: bool);

        #[method(includesTimeRemainingPhrase)]
        pub unsafe fn includesTimeRemainingPhrase(&self) -> bool;

        #[method(setIncludesTimeRemainingPhrase:)]
        pub unsafe fn setIncludesTimeRemainingPhrase(&self, includes_time_remaining_phrase: bool);

        #[method(formattingContext)]
        pub unsafe fn formattingContext(&self) -> NSFormattingContext;

        #[method(setFormattingContext:)]
        pub unsafe fn setFormattingContext(&self, formatting_context: NSFormattingContext);

        #[cfg(feature = "Foundation_NSString")]
        #[method(getObjectValue:forString:errorDescription:)]
        pub unsafe fn getObjectValue_forString_errorDescription(
            &self,
            obj: Option<&mut Option<Id<AnyObject>>>,
            string: &NSString,
            error: Option<&mut Option<Id<NSString>>>,
        ) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSDateComponentsFormatter")]
    unsafe impl NSDateComponentsFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

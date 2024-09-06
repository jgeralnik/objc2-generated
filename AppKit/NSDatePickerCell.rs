//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDatePickerStyle(pub NSUInteger);
impl NSDatePickerStyle {
    #[doc(alias = "NSDatePickerStyleTextFieldAndStepper")]
    pub const TextFieldAndStepper: Self = Self(0);
    #[doc(alias = "NSDatePickerStyleClockAndCalendar")]
    pub const ClockAndCalendar: Self = Self(1);
    #[doc(alias = "NSDatePickerStyleTextField")]
    pub const TextField: Self = Self(2);
}

unsafe impl Encode for NSDatePickerStyle {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDatePickerStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDatePickerMode(pub NSUInteger);
impl NSDatePickerMode {
    #[doc(alias = "NSDatePickerModeSingle")]
    pub const Single: Self = Self(0);
    #[doc(alias = "NSDatePickerModeRange")]
    pub const Range: Self = Self(1);
}

unsafe impl Encode for NSDatePickerMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDatePickerMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDatePickerElementFlags(pub NSUInteger);
bitflags::bitflags! {
    impl NSDatePickerElementFlags: NSUInteger {
        const NSDatePickerElementFlagHourMinute = 0x000c;
        const NSDatePickerElementFlagHourMinuteSecond = 0x000e;
        const NSDatePickerElementFlagTimeZone = 0x0010;
        const NSDatePickerElementFlagYearMonth = 0x00c0;
        const NSDatePickerElementFlagYearMonthDay = 0x00e0;
        const NSDatePickerElementFlagEra = 0x0100;
    }
}

unsafe impl Encode for NSDatePickerElementFlags {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDatePickerElementFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    pub struct NSDatePickerCell;

    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    unsafe impl ClassType for NSDatePickerCell {
        #[inherits(NSCell, NSObject)]
        type Super = NSActionCell;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSActionCell",
    feature = "NSCell"
))]
unsafe impl NSAccessibility for NSDatePickerCell {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSActionCell",
    feature = "NSCell"
))]
unsafe impl NSAccessibilityElementProtocol for NSDatePickerCell {}

#[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
unsafe impl NSCoding for NSDatePickerCell {}

#[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
unsafe impl NSCopying for NSDatePickerCell {}

#[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
unsafe impl CopyingHelper for NSDatePickerCell {
    type Result = Self;
}

#[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
unsafe impl NSObjectProtocol for NSDatePickerCell {}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSUserInterfaceItemIdentification"
))]
unsafe impl NSUserInterfaceItemIdentification for NSDatePickerCell {}

extern_methods!(
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    unsafe impl NSDatePickerCell {
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(this: Allocated<Self>, string: &NSString) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Allocated<Self>,
            image: Option<&NSImage>,
        ) -> Retained<Self>;

        #[method(datePickerStyle)]
        pub unsafe fn datePickerStyle(&self) -> NSDatePickerStyle;

        #[method(setDatePickerStyle:)]
        pub unsafe fn setDatePickerStyle(&self, date_picker_style: NSDatePickerStyle);

        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;

        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, draws_background: bool);

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Retained<NSColor>;

        #[cfg(feature = "NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: &NSColor);

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other textColor)]
        pub unsafe fn textColor(&self) -> Retained<NSColor>;

        #[cfg(feature = "NSColor")]
        #[method(setTextColor:)]
        pub unsafe fn setTextColor(&self, text_color: &NSColor);

        #[method(datePickerMode)]
        pub unsafe fn datePickerMode(&self) -> NSDatePickerMode;

        #[method(setDatePickerMode:)]
        pub unsafe fn setDatePickerMode(&self, date_picker_mode: NSDatePickerMode);

        #[method(datePickerElements)]
        pub unsafe fn datePickerElements(&self) -> NSDatePickerElementFlags;

        #[method(setDatePickerElements:)]
        pub unsafe fn setDatePickerElements(&self, date_picker_elements: NSDatePickerElementFlags);

        #[method_id(@__retain_semantics Other calendar)]
        pub unsafe fn calendar(&self) -> Option<Retained<NSCalendar>>;

        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>);

        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Option<Retained<NSLocale>>;

        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Option<Retained<NSTimeZone>>;

        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, time_zone: Option<&NSTimeZone>);

        #[method_id(@__retain_semantics Other dateValue)]
        pub unsafe fn dateValue(&self) -> Retained<NSDate>;

        #[method(setDateValue:)]
        pub unsafe fn setDateValue(&self, date_value: &NSDate);

        #[method(timeInterval)]
        pub unsafe fn timeInterval(&self) -> NSTimeInterval;

        #[method(setTimeInterval:)]
        pub unsafe fn setTimeInterval(&self, time_interval: NSTimeInterval);

        #[method_id(@__retain_semantics Other minDate)]
        pub unsafe fn minDate(&self) -> Option<Retained<NSDate>>;

        #[method(setMinDate:)]
        pub unsafe fn setMinDate(&self, min_date: Option<&NSDate>);

        #[method_id(@__retain_semantics Other maxDate)]
        pub unsafe fn maxDate(&self) -> Option<Retained<NSDate>>;

        #[method(setMaxDate:)]
        pub unsafe fn setMaxDate(&self, max_date: Option<&NSDate>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSDatePickerCellDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSDatePickerCellDelegate>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    unsafe impl NSDatePickerCell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    unsafe impl NSDatePickerCell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSDatePickerCellDelegate: NSObjectProtocol + IsMainThreadOnly {
        #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
        #[optional]
        #[method(datePickerCell:validateProposedDateValue:timeInterval:)]
        unsafe fn datePickerCell_validateProposedDateValue_timeInterval(
            &self,
            date_picker_cell: &NSDatePickerCell,
            proposed_date_value: &mut Retained<NSDate>,
            proposed_time_interval: *mut NSTimeInterval,
        );
    }

    unsafe impl ProtocolType for dyn NSDatePickerCellDelegate {}
);

pub static NSTextFieldAndStepperDatePickerStyle: NSDatePickerStyle =
    NSDatePickerStyle(NSDatePickerStyle::TextFieldAndStepper.0);

pub static NSClockAndCalendarDatePickerStyle: NSDatePickerStyle =
    NSDatePickerStyle(NSDatePickerStyle::ClockAndCalendar.0);

pub static NSTextFieldDatePickerStyle: NSDatePickerStyle =
    NSDatePickerStyle(NSDatePickerStyle::TextField.0);

pub static NSSingleDateMode: NSDatePickerMode = NSDatePickerMode(NSDatePickerMode::Single.0);

pub static NSRangeDateMode: NSDatePickerMode = NSDatePickerMode(NSDatePickerMode::Range.0);

pub static NSHourMinuteDatePickerElementFlag: NSDatePickerElementFlags =
    NSDatePickerElementFlags(NSDatePickerElementFlags::NSDatePickerElementFlagHourMinute.0);

pub static NSHourMinuteSecondDatePickerElementFlag: NSDatePickerElementFlags =
    NSDatePickerElementFlags(NSDatePickerElementFlags::NSDatePickerElementFlagHourMinuteSecond.0);

pub static NSTimeZoneDatePickerElementFlag: NSDatePickerElementFlags =
    NSDatePickerElementFlags(NSDatePickerElementFlags::NSDatePickerElementFlagTimeZone.0);

pub static NSYearMonthDatePickerElementFlag: NSDatePickerElementFlags =
    NSDatePickerElementFlags(NSDatePickerElementFlags::NSDatePickerElementFlagYearMonth.0);

pub static NSYearMonthDayDatePickerElementFlag: NSDatePickerElementFlags =
    NSDatePickerElementFlags(NSDatePickerElementFlags::NSDatePickerElementFlagYearMonthDay.0);

pub static NSEraDatePickerElementFlag: NSDatePickerElementFlags =
    NSDatePickerElementFlags(NSDatePickerElementFlags::NSDatePickerElementFlagEra.0);

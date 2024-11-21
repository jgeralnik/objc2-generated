//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "NSString")]
pub type NSCalendarIdentifier = NSString;

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierGregorian: &'static NSCalendarIdentifier;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierBuddhist: &'static NSCalendarIdentifier;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierChinese: &'static NSCalendarIdentifier;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierCoptic: &'static NSCalendarIdentifier;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierEthiopicAmeteMihret: &'static NSCalendarIdentifier;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierEthiopicAmeteAlem: &'static NSCalendarIdentifier;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierHebrew: &'static NSCalendarIdentifier;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierISO8601: &'static NSCalendarIdentifier;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierIndian: &'static NSCalendarIdentifier;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierIslamic: &'static NSCalendarIdentifier;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierIslamicCivil: &'static NSCalendarIdentifier;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierJapanese: &'static NSCalendarIdentifier;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierPersian: &'static NSCalendarIdentifier;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierRepublicOfChina: &'static NSCalendarIdentifier;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierIslamicTabular: &'static NSCalendarIdentifier;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSCalendarIdentifierIslamicUmmAlQura: &'static NSCalendarIdentifier;
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSCalendarUnit(pub NSUInteger);
bitflags::bitflags! {
    impl NSCalendarUnit: NSUInteger {
        #[doc(alias = "NSCalendarUnitEra")]
        const Era = 2;
        #[doc(alias = "NSCalendarUnitYear")]
        const Year = 4;
        #[doc(alias = "NSCalendarUnitMonth")]
        const Month = 8;
        #[doc(alias = "NSCalendarUnitDay")]
        const Day = 16;
        #[doc(alias = "NSCalendarUnitHour")]
        const Hour = 32;
        #[doc(alias = "NSCalendarUnitMinute")]
        const Minute = 64;
        #[doc(alias = "NSCalendarUnitSecond")]
        const Second = 128;
        #[doc(alias = "NSCalendarUnitWeekday")]
        const Weekday = 512;
        #[doc(alias = "NSCalendarUnitWeekdayOrdinal")]
        const WeekdayOrdinal = 1024;
        #[doc(alias = "NSCalendarUnitQuarter")]
        const Quarter = 2048;
        #[doc(alias = "NSCalendarUnitWeekOfMonth")]
        const WeekOfMonth = 4096;
        #[doc(alias = "NSCalendarUnitWeekOfYear")]
        const WeekOfYear = 8192;
        #[doc(alias = "NSCalendarUnitYearForWeekOfYear")]
        const YearForWeekOfYear = 16384;
        #[doc(alias = "NSCalendarUnitNanosecond")]
        const Nanosecond = 32768;
        #[doc(alias = "NSCalendarUnitDayOfYear")]
        const DayOfYear = 65536;
        #[doc(alias = "NSCalendarUnitCalendar")]
        const Calendar = 1048576;
        #[doc(alias = "NSCalendarUnitTimeZone")]
        const TimeZone = 2097152;
#[deprecated]
        const NSEraCalendarUnit = 2;
#[deprecated]
        const NSYearCalendarUnit = 4;
#[deprecated]
        const NSMonthCalendarUnit = 8;
#[deprecated]
        const NSDayCalendarUnit = 16;
#[deprecated]
        const NSHourCalendarUnit = 32;
#[deprecated]
        const NSMinuteCalendarUnit = 64;
#[deprecated]
        const NSSecondCalendarUnit = 128;
#[deprecated = "NSCalendarUnitWeekOfMonth or NSCalendarUnitWeekOfYear, depending on which you mean"]
        const NSWeekCalendarUnit = 256;
#[deprecated]
        const NSWeekdayCalendarUnit = 512;
#[deprecated]
        const NSWeekdayOrdinalCalendarUnit = 1024;
#[deprecated]
        const NSQuarterCalendarUnit = 2048;
#[deprecated]
        const NSWeekOfMonthCalendarUnit = 4096;
#[deprecated]
        const NSWeekOfYearCalendarUnit = 8192;
#[deprecated]
        const NSYearForWeekOfYearCalendarUnit = 16384;
#[deprecated]
        const NSCalendarCalendarUnit = 1048576;
#[deprecated]
        const NSTimeZoneCalendarUnit = 2097152;
    }
}

unsafe impl Encode for NSCalendarUnit {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSCalendarUnit {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSCalendarOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSCalendarOptions: NSUInteger {
        const NSCalendarWrapComponents = 1<<0;
        const NSCalendarMatchStrictly = 1<<1;
        const NSCalendarSearchBackwards = 1<<2;
        const NSCalendarMatchPreviousTimePreservingSmallerUnits = 1<<8;
        const NSCalendarMatchNextTimePreservingSmallerUnits = 1<<9;
        const NSCalendarMatchNextTime = 1<<10;
        const NSCalendarMatchFirst = 1<<12;
        const NSCalendarMatchLast = 1<<13;
    }
}

unsafe impl Encode for NSCalendarOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSCalendarOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCalendar;
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSCalendar {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSCalendar {}

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSCalendar {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSCalendar {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSCalendar {}

extern_methods!(
    unsafe impl NSCalendar {
        #[method_id(@__retain_semantics Other currentCalendar)]
        pub unsafe fn currentCalendar() -> Retained<NSCalendar>;

        #[method_id(@__retain_semantics Other autoupdatingCurrentCalendar)]
        pub unsafe fn autoupdatingCurrentCalendar() -> Retained<NSCalendar>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other calendarWithIdentifier:)]
        pub unsafe fn calendarWithIdentifier(
            calendar_identifier_constant: &NSCalendarIdentifier,
        ) -> Option<Retained<NSCalendar>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Init initWithCalendarIdentifier:)]
        pub unsafe fn initWithCalendarIdentifier(
            this: Allocated<Self>,
            ident: &NSCalendarIdentifier,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other calendarIdentifier)]
        pub unsafe fn calendarIdentifier(&self) -> Retained<NSCalendarIdentifier>;

        #[cfg(feature = "NSLocale")]
        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Option<Retained<NSLocale>>;

        #[cfg(feature = "NSLocale")]
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[cfg(feature = "NSTimeZone")]
        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Retained<NSTimeZone>;

        #[cfg(feature = "NSTimeZone")]
        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, time_zone: &NSTimeZone);

        #[method(firstWeekday)]
        pub unsafe fn firstWeekday(&self) -> NSUInteger;

        #[method(setFirstWeekday:)]
        pub unsafe fn setFirstWeekday(&self, first_weekday: NSUInteger);

        #[method(minimumDaysInFirstWeek)]
        pub unsafe fn minimumDaysInFirstWeek(&self) -> NSUInteger;

        #[method(setMinimumDaysInFirstWeek:)]
        pub unsafe fn setMinimumDaysInFirstWeek(&self, minimum_days_in_first_week: NSUInteger);

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other eraSymbols)]
        pub unsafe fn eraSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other longEraSymbols)]
        pub unsafe fn longEraSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other monthSymbols)]
        pub unsafe fn monthSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other shortMonthSymbols)]
        pub unsafe fn shortMonthSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other veryShortMonthSymbols)]
        pub unsafe fn veryShortMonthSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other standaloneMonthSymbols)]
        pub unsafe fn standaloneMonthSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other shortStandaloneMonthSymbols)]
        pub unsafe fn shortStandaloneMonthSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other veryShortStandaloneMonthSymbols)]
        pub unsafe fn veryShortStandaloneMonthSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other weekdaySymbols)]
        pub unsafe fn weekdaySymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other shortWeekdaySymbols)]
        pub unsafe fn shortWeekdaySymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other veryShortWeekdaySymbols)]
        pub unsafe fn veryShortWeekdaySymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other standaloneWeekdaySymbols)]
        pub unsafe fn standaloneWeekdaySymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other shortStandaloneWeekdaySymbols)]
        pub unsafe fn shortStandaloneWeekdaySymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other veryShortStandaloneWeekdaySymbols)]
        pub unsafe fn veryShortStandaloneWeekdaySymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other quarterSymbols)]
        pub unsafe fn quarterSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other shortQuarterSymbols)]
        pub unsafe fn shortQuarterSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other standaloneQuarterSymbols)]
        pub unsafe fn standaloneQuarterSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other shortStandaloneQuarterSymbols)]
        pub unsafe fn shortStandaloneQuarterSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other AMSymbol)]
        pub unsafe fn AMSymbol(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other PMSymbol)]
        pub unsafe fn PMSymbol(&self) -> Retained<NSString>;

        #[cfg(feature = "NSRange")]
        #[method(minimumRangeOfUnit:)]
        pub unsafe fn minimumRangeOfUnit(&self, unit: NSCalendarUnit) -> NSRange;

        #[cfg(feature = "NSRange")]
        #[method(maximumRangeOfUnit:)]
        pub unsafe fn maximumRangeOfUnit(&self, unit: NSCalendarUnit) -> NSRange;

        #[cfg(all(feature = "NSDate", feature = "NSRange"))]
        #[method(rangeOfUnit:inUnit:forDate:)]
        pub unsafe fn rangeOfUnit_inUnit_forDate(
            &self,
            smaller: NSCalendarUnit,
            larger: NSCalendarUnit,
            date: &NSDate,
        ) -> NSRange;

        #[cfg(feature = "NSDate")]
        #[method(ordinalityOfUnit:inUnit:forDate:)]
        pub unsafe fn ordinalityOfUnit_inUnit_forDate(
            &self,
            smaller: NSCalendarUnit,
            larger: NSCalendarUnit,
            date: &NSDate,
        ) -> NSUInteger;

        #[cfg(feature = "NSDate")]
        #[method(rangeOfUnit:startDate:interval:forDate:)]
        pub unsafe fn rangeOfUnit_startDate_interval_forDate(
            &self,
            unit: NSCalendarUnit,
            datep: Option<&mut Option<Retained<NSDate>>>,
            tip: *mut NSTimeInterval,
            date: &NSDate,
        ) -> bool;

        #[cfg(feature = "NSDate")]
        #[method_id(@__retain_semantics Other dateFromComponents:)]
        pub unsafe fn dateFromComponents(
            &self,
            comps: &NSDateComponents,
        ) -> Option<Retained<NSDate>>;

        #[cfg(feature = "NSDate")]
        #[method_id(@__retain_semantics Other components:fromDate:)]
        pub unsafe fn components_fromDate(
            &self,
            unit_flags: NSCalendarUnit,
            date: &NSDate,
        ) -> Retained<NSDateComponents>;

        #[cfg(feature = "NSDate")]
        #[method_id(@__retain_semantics Other dateByAddingComponents:toDate:options:)]
        pub unsafe fn dateByAddingComponents_toDate_options(
            &self,
            comps: &NSDateComponents,
            date: &NSDate,
            opts: NSCalendarOptions,
        ) -> Option<Retained<NSDate>>;

        #[cfg(feature = "NSDate")]
        #[method_id(@__retain_semantics Other components:fromDate:toDate:options:)]
        pub unsafe fn components_fromDate_toDate_options(
            &self,
            unit_flags: NSCalendarUnit,
            starting_date: &NSDate,
            result_date: &NSDate,
            opts: NSCalendarOptions,
        ) -> Retained<NSDateComponents>;

        #[cfg(feature = "NSDate")]
        #[method(getEra:year:month:day:fromDate:)]
        pub unsafe fn getEra_year_month_day_fromDate(
            &self,
            era_value_pointer: *mut NSInteger,
            year_value_pointer: *mut NSInteger,
            month_value_pointer: *mut NSInteger,
            day_value_pointer: *mut NSInteger,
            date: &NSDate,
        );

        #[cfg(feature = "NSDate")]
        #[method(getEra:yearForWeekOfYear:weekOfYear:weekday:fromDate:)]
        pub unsafe fn getEra_yearForWeekOfYear_weekOfYear_weekday_fromDate(
            &self,
            era_value_pointer: *mut NSInteger,
            year_value_pointer: *mut NSInteger,
            week_value_pointer: *mut NSInteger,
            weekday_value_pointer: *mut NSInteger,
            date: &NSDate,
        );

        #[cfg(feature = "NSDate")]
        #[method(getHour:minute:second:nanosecond:fromDate:)]
        pub unsafe fn getHour_minute_second_nanosecond_fromDate(
            &self,
            hour_value_pointer: *mut NSInteger,
            minute_value_pointer: *mut NSInteger,
            second_value_pointer: *mut NSInteger,
            nanosecond_value_pointer: *mut NSInteger,
            date: &NSDate,
        );

        #[cfg(feature = "NSDate")]
        #[method(component:fromDate:)]
        pub unsafe fn component_fromDate(&self, unit: NSCalendarUnit, date: &NSDate) -> NSInteger;

        #[cfg(feature = "NSDate")]
        #[method_id(@__retain_semantics Other dateWithEra:year:month:day:hour:minute:second:nanosecond:)]
        pub unsafe fn dateWithEra_year_month_day_hour_minute_second_nanosecond(
            &self,
            era_value: NSInteger,
            year_value: NSInteger,
            month_value: NSInteger,
            day_value: NSInteger,
            hour_value: NSInteger,
            minute_value: NSInteger,
            second_value: NSInteger,
            nanosecond_value: NSInteger,
        ) -> Option<Retained<NSDate>>;

        #[cfg(feature = "NSDate")]
        #[method_id(@__retain_semantics Other dateWithEra:yearForWeekOfYear:weekOfYear:weekday:hour:minute:second:nanosecond:)]
        pub unsafe fn dateWithEra_yearForWeekOfYear_weekOfYear_weekday_hour_minute_second_nanosecond(
            &self,
            era_value: NSInteger,
            year_value: NSInteger,
            week_value: NSInteger,
            weekday_value: NSInteger,
            hour_value: NSInteger,
            minute_value: NSInteger,
            second_value: NSInteger,
            nanosecond_value: NSInteger,
        ) -> Option<Retained<NSDate>>;

        #[cfg(feature = "NSDate")]
        #[method_id(@__retain_semantics Other startOfDayForDate:)]
        pub unsafe fn startOfDayForDate(&self, date: &NSDate) -> Retained<NSDate>;

        #[cfg(all(feature = "NSDate", feature = "NSTimeZone"))]
        #[method_id(@__retain_semantics Other componentsInTimeZone:fromDate:)]
        pub unsafe fn componentsInTimeZone_fromDate(
            &self,
            timezone: &NSTimeZone,
            date: &NSDate,
        ) -> Retained<NSDateComponents>;

        #[cfg(all(feature = "NSDate", feature = "NSObjCRuntime"))]
        #[method(compareDate:toDate:toUnitGranularity:)]
        pub unsafe fn compareDate_toDate_toUnitGranularity(
            &self,
            date1: &NSDate,
            date2: &NSDate,
            unit: NSCalendarUnit,
        ) -> NSComparisonResult;

        #[cfg(feature = "NSDate")]
        #[method(isDate:equalToDate:toUnitGranularity:)]
        pub unsafe fn isDate_equalToDate_toUnitGranularity(
            &self,
            date1: &NSDate,
            date2: &NSDate,
            unit: NSCalendarUnit,
        ) -> bool;

        #[cfg(feature = "NSDate")]
        #[method(isDate:inSameDayAsDate:)]
        pub unsafe fn isDate_inSameDayAsDate(&self, date1: &NSDate, date2: &NSDate) -> bool;

        #[cfg(feature = "NSDate")]
        #[method(isDateInToday:)]
        pub unsafe fn isDateInToday(&self, date: &NSDate) -> bool;

        #[cfg(feature = "NSDate")]
        #[method(isDateInYesterday:)]
        pub unsafe fn isDateInYesterday(&self, date: &NSDate) -> bool;

        #[cfg(feature = "NSDate")]
        #[method(isDateInTomorrow:)]
        pub unsafe fn isDateInTomorrow(&self, date: &NSDate) -> bool;

        #[cfg(feature = "NSDate")]
        #[method(isDateInWeekend:)]
        pub unsafe fn isDateInWeekend(&self, date: &NSDate) -> bool;

        #[cfg(feature = "NSDate")]
        #[method(rangeOfWeekendStartDate:interval:containingDate:)]
        pub unsafe fn rangeOfWeekendStartDate_interval_containingDate(
            &self,
            datep: Option<&mut Option<Retained<NSDate>>>,
            tip: *mut NSTimeInterval,
            date: &NSDate,
        ) -> bool;

        #[cfg(feature = "NSDate")]
        #[method(nextWeekendStartDate:interval:options:afterDate:)]
        pub unsafe fn nextWeekendStartDate_interval_options_afterDate(
            &self,
            datep: Option<&mut Option<Retained<NSDate>>>,
            tip: *mut NSTimeInterval,
            options: NSCalendarOptions,
            date: &NSDate,
        ) -> bool;

        #[method_id(@__retain_semantics Other components:fromDateComponents:toDateComponents:options:)]
        pub unsafe fn components_fromDateComponents_toDateComponents_options(
            &self,
            unit_flags: NSCalendarUnit,
            starting_date_comp: &NSDateComponents,
            result_date_comp: &NSDateComponents,
            options: NSCalendarOptions,
        ) -> Retained<NSDateComponents>;

        #[cfg(feature = "NSDate")]
        #[method_id(@__retain_semantics Other dateByAddingUnit:value:toDate:options:)]
        pub unsafe fn dateByAddingUnit_value_toDate_options(
            &self,
            unit: NSCalendarUnit,
            value: NSInteger,
            date: &NSDate,
            options: NSCalendarOptions,
        ) -> Option<Retained<NSDate>>;

        #[cfg(all(feature = "NSDate", feature = "block2"))]
        #[method(enumerateDatesStartingAfterDate:matchingComponents:options:usingBlock:)]
        pub unsafe fn enumerateDatesStartingAfterDate_matchingComponents_options_usingBlock(
            &self,
            start: &NSDate,
            comps: &NSDateComponents,
            opts: NSCalendarOptions,
            block: &block2::Block<dyn Fn(*mut NSDate, Bool, NonNull<Bool>) + '_>,
        );

        #[cfg(feature = "NSDate")]
        #[method_id(@__retain_semantics Other nextDateAfterDate:matchingComponents:options:)]
        pub unsafe fn nextDateAfterDate_matchingComponents_options(
            &self,
            date: &NSDate,
            comps: &NSDateComponents,
            options: NSCalendarOptions,
        ) -> Option<Retained<NSDate>>;

        #[cfg(feature = "NSDate")]
        #[method_id(@__retain_semantics Other nextDateAfterDate:matchingUnit:value:options:)]
        pub unsafe fn nextDateAfterDate_matchingUnit_value_options(
            &self,
            date: &NSDate,
            unit: NSCalendarUnit,
            value: NSInteger,
            options: NSCalendarOptions,
        ) -> Option<Retained<NSDate>>;

        #[cfg(feature = "NSDate")]
        #[method_id(@__retain_semantics Other nextDateAfterDate:matchingHour:minute:second:options:)]
        pub unsafe fn nextDateAfterDate_matchingHour_minute_second_options(
            &self,
            date: &NSDate,
            hour_value: NSInteger,
            minute_value: NSInteger,
            second_value: NSInteger,
            options: NSCalendarOptions,
        ) -> Option<Retained<NSDate>>;

        #[cfg(feature = "NSDate")]
        #[method_id(@__retain_semantics Other dateBySettingUnit:value:ofDate:options:)]
        pub unsafe fn dateBySettingUnit_value_ofDate_options(
            &self,
            unit: NSCalendarUnit,
            v: NSInteger,
            date: &NSDate,
            opts: NSCalendarOptions,
        ) -> Option<Retained<NSDate>>;

        #[cfg(feature = "NSDate")]
        #[method_id(@__retain_semantics Other dateBySettingHour:minute:second:ofDate:options:)]
        pub unsafe fn dateBySettingHour_minute_second_ofDate_options(
            &self,
            h: NSInteger,
            m: NSInteger,
            s: NSInteger,
            date: &NSDate,
            opts: NSCalendarOptions,
        ) -> Option<Retained<NSDate>>;

        #[cfg(feature = "NSDate")]
        #[method(date:matchesComponents:)]
        pub unsafe fn date_matchesComponents(
            &self,
            date: &NSDate,
            components: &NSDateComponents,
        ) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSCalendar {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSCalendarDayChangedNotification: &'static NSNotificationName;
}

pub const NSDateComponentUndefined: NSInteger = NSIntegerMax as _;
#[deprecated]
pub const NSUndefinedDateComponent: NSInteger = NSDateComponentUndefined;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDateComponents;
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSDateComponents {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSDateComponents {}

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSDateComponents {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSDateComponents {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSDateComponents {}

extern_methods!(
    unsafe impl NSDateComponents {
        #[method_id(@__retain_semantics Other calendar)]
        pub unsafe fn calendar(&self) -> Option<Retained<NSCalendar>>;

        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>);

        #[cfg(feature = "NSTimeZone")]
        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Option<Retained<NSTimeZone>>;

        #[cfg(feature = "NSTimeZone")]
        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, time_zone: Option<&NSTimeZone>);

        #[method(era)]
        pub unsafe fn era(&self) -> NSInteger;

        #[method(setEra:)]
        pub unsafe fn setEra(&self, era: NSInteger);

        #[method(year)]
        pub unsafe fn year(&self) -> NSInteger;

        #[method(setYear:)]
        pub unsafe fn setYear(&self, year: NSInteger);

        #[method(month)]
        pub unsafe fn month(&self) -> NSInteger;

        #[method(setMonth:)]
        pub unsafe fn setMonth(&self, month: NSInteger);

        #[method(day)]
        pub unsafe fn day(&self) -> NSInteger;

        #[method(setDay:)]
        pub unsafe fn setDay(&self, day: NSInteger);

        #[method(hour)]
        pub unsafe fn hour(&self) -> NSInteger;

        #[method(setHour:)]
        pub unsafe fn setHour(&self, hour: NSInteger);

        #[method(minute)]
        pub unsafe fn minute(&self) -> NSInteger;

        #[method(setMinute:)]
        pub unsafe fn setMinute(&self, minute: NSInteger);

        #[method(second)]
        pub unsafe fn second(&self) -> NSInteger;

        #[method(setSecond:)]
        pub unsafe fn setSecond(&self, second: NSInteger);

        #[method(nanosecond)]
        pub unsafe fn nanosecond(&self) -> NSInteger;

        #[method(setNanosecond:)]
        pub unsafe fn setNanosecond(&self, nanosecond: NSInteger);

        #[method(weekday)]
        pub unsafe fn weekday(&self) -> NSInteger;

        #[method(setWeekday:)]
        pub unsafe fn setWeekday(&self, weekday: NSInteger);

        #[method(weekdayOrdinal)]
        pub unsafe fn weekdayOrdinal(&self) -> NSInteger;

        #[method(setWeekdayOrdinal:)]
        pub unsafe fn setWeekdayOrdinal(&self, weekday_ordinal: NSInteger);

        #[method(quarter)]
        pub unsafe fn quarter(&self) -> NSInteger;

        #[method(setQuarter:)]
        pub unsafe fn setQuarter(&self, quarter: NSInteger);

        #[method(weekOfMonth)]
        pub unsafe fn weekOfMonth(&self) -> NSInteger;

        #[method(setWeekOfMonth:)]
        pub unsafe fn setWeekOfMonth(&self, week_of_month: NSInteger);

        #[method(weekOfYear)]
        pub unsafe fn weekOfYear(&self) -> NSInteger;

        #[method(setWeekOfYear:)]
        pub unsafe fn setWeekOfYear(&self, week_of_year: NSInteger);

        #[method(yearForWeekOfYear)]
        pub unsafe fn yearForWeekOfYear(&self) -> NSInteger;

        #[method(setYearForWeekOfYear:)]
        pub unsafe fn setYearForWeekOfYear(&self, year_for_week_of_year: NSInteger);

        #[method(dayOfYear)]
        pub unsafe fn dayOfYear(&self) -> NSInteger;

        #[method(setDayOfYear:)]
        pub unsafe fn setDayOfYear(&self, day_of_year: NSInteger);

        #[method(isLeapMonth)]
        pub unsafe fn isLeapMonth(&self) -> bool;

        #[method(setLeapMonth:)]
        pub unsafe fn setLeapMonth(&self, leap_month: bool);

        #[cfg(feature = "NSDate")]
        #[method_id(@__retain_semantics Other date)]
        pub unsafe fn date(&self) -> Option<Retained<NSDate>>;

        #[deprecated = "Use -weekOfMonth or -weekOfYear, depending on which you mean"]
        #[method(week)]
        pub unsafe fn week(&self) -> NSInteger;

        #[deprecated = "Use -setWeekOfMonth: or -setWeekOfYear:, depending on which you mean"]
        #[method(setWeek:)]
        pub unsafe fn setWeek(&self, v: NSInteger);

        #[method(setValue:forComponent:)]
        pub unsafe fn setValue_forComponent(&self, value: NSInteger, unit: NSCalendarUnit);

        #[method(valueForComponent:)]
        pub unsafe fn valueForComponent(&self, unit: NSCalendarUnit) -> NSInteger;

        #[method(isValidDate)]
        pub unsafe fn isValidDate(&self) -> bool;

        #[method(isValidDateInCalendar:)]
        pub unsafe fn isValidDateInCalendar(&self, calendar: &NSCalendar) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSDateComponents {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

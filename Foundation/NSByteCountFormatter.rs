//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSByteCountFormatterUnits(pub NSUInteger);
bitflags::bitflags! {
    impl NSByteCountFormatterUnits: NSUInteger {
        const NSByteCountFormatterUseDefault = 0;
        const NSByteCountFormatterUseBytes = 1<<0;
        const NSByteCountFormatterUseKB = 1<<1;
        const NSByteCountFormatterUseMB = 1<<2;
        const NSByteCountFormatterUseGB = 1<<3;
        const NSByteCountFormatterUseTB = 1<<4;
        const NSByteCountFormatterUsePB = 1<<5;
        const NSByteCountFormatterUseEB = 1<<6;
        const NSByteCountFormatterUseZB = 1<<7;
        const NSByteCountFormatterUseYBOrHigher = 0x0FF<<8;
        const NSByteCountFormatterUseAll = 0x0FFFF;
    }
}

unsafe impl Encode for NSByteCountFormatterUnits {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSByteCountFormatterUnits {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSByteCountFormatterCountStyle(pub NSInteger);
impl NSByteCountFormatterCountStyle {
    #[doc(alias = "NSByteCountFormatterCountStyleFile")]
    pub const File: Self = Self(0);
    #[doc(alias = "NSByteCountFormatterCountStyleMemory")]
    pub const Memory: Self = Self(1);
    #[doc(alias = "NSByteCountFormatterCountStyleDecimal")]
    pub const Decimal: Self = Self(2);
    #[doc(alias = "NSByteCountFormatterCountStyleBinary")]
    pub const Binary: Self = Self(3);
}

unsafe impl Encode for NSByteCountFormatterCountStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSByteCountFormatterCountStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(NSFormatter, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSFormatter")]
    pub struct NSByteCountFormatter;
);

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl NSCoding for NSByteCountFormatter {}

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl NSCopying for NSByteCountFormatter {}

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl CopyingHelper for NSByteCountFormatter {
    type Result = Self;
}

#[cfg(feature = "NSFormatter")]
unsafe impl NSObjectProtocol for NSByteCountFormatter {}

extern_methods!(
    #[cfg(feature = "NSFormatter")]
    unsafe impl NSByteCountFormatter {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other stringFromByteCount:countStyle:)]
        pub unsafe fn stringFromByteCount_countStyle(
            byte_count: c_longlong,
            count_style: NSByteCountFormatterCountStyle,
        ) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other stringFromByteCount:)]
        pub unsafe fn stringFromByteCount(&self, byte_count: c_longlong) -> Retained<NSString>;

        #[cfg(all(feature = "NSMeasurement", feature = "NSString", feature = "NSUnit"))]
        #[method_id(@__retain_semantics Other stringFromMeasurement:countStyle:)]
        pub unsafe fn stringFromMeasurement_countStyle(
            measurement: &NSMeasurement<NSUnitInformationStorage>,
            count_style: NSByteCountFormatterCountStyle,
        ) -> Retained<NSString>;

        #[cfg(all(feature = "NSMeasurement", feature = "NSString", feature = "NSUnit"))]
        #[method_id(@__retain_semantics Other stringFromMeasurement:)]
        pub unsafe fn stringFromMeasurement(
            &self,
            measurement: &NSMeasurement<NSUnitInformationStorage>,
        ) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other stringForObjectValue:)]
        pub unsafe fn stringForObjectValue(
            &self,
            obj: Option<&AnyObject>,
        ) -> Option<Retained<NSString>>;

        #[method(allowedUnits)]
        pub unsafe fn allowedUnits(&self) -> NSByteCountFormatterUnits;

        #[method(setAllowedUnits:)]
        pub unsafe fn setAllowedUnits(&self, allowed_units: NSByteCountFormatterUnits);

        #[method(countStyle)]
        pub unsafe fn countStyle(&self) -> NSByteCountFormatterCountStyle;

        #[method(setCountStyle:)]
        pub unsafe fn setCountStyle(&self, count_style: NSByteCountFormatterCountStyle);

        #[method(allowsNonnumericFormatting)]
        pub unsafe fn allowsNonnumericFormatting(&self) -> bool;

        #[method(setAllowsNonnumericFormatting:)]
        pub unsafe fn setAllowsNonnumericFormatting(&self, allows_nonnumeric_formatting: bool);

        #[method(includesUnit)]
        pub unsafe fn includesUnit(&self) -> bool;

        #[method(setIncludesUnit:)]
        pub unsafe fn setIncludesUnit(&self, includes_unit: bool);

        #[method(includesCount)]
        pub unsafe fn includesCount(&self) -> bool;

        #[method(setIncludesCount:)]
        pub unsafe fn setIncludesCount(&self, includes_count: bool);

        #[method(includesActualByteCount)]
        pub unsafe fn includesActualByteCount(&self) -> bool;

        #[method(setIncludesActualByteCount:)]
        pub unsafe fn setIncludesActualByteCount(&self, includes_actual_byte_count: bool);

        #[method(isAdaptive)]
        pub unsafe fn isAdaptive(&self) -> bool;

        #[method(setAdaptive:)]
        pub unsafe fn setAdaptive(&self, adaptive: bool);

        #[method(zeroPadsFractionDigits)]
        pub unsafe fn zeroPadsFractionDigits(&self) -> bool;

        #[method(setZeroPadsFractionDigits:)]
        pub unsafe fn setZeroPadsFractionDigits(&self, zero_pads_fraction_digits: bool);

        #[method(formattingContext)]
        pub unsafe fn formattingContext(&self) -> NSFormattingContext;

        #[method(setFormattingContext:)]
        pub unsafe fn setFormattingContext(&self, formatting_context: NSFormattingContext);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSFormatter")]
    unsafe impl NSByteCountFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

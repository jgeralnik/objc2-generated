//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSMeasurementFormatterUnitOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSMeasurementFormatterUnitOptions: NSUInteger {
        #[doc(alias = "NSMeasurementFormatterUnitOptionsProvidedUnit")]
        const ProvidedUnit = 1<<0;
        #[doc(alias = "NSMeasurementFormatterUnitOptionsNaturalScale")]
        const NaturalScale = 1<<1;
        #[doc(alias = "NSMeasurementFormatterUnitOptionsTemperatureWithoutUnit")]
        const TemperatureWithoutUnit = 1<<2;
    }
}

unsafe impl Encode for NSMeasurementFormatterUnitOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSMeasurementFormatterUnitOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSFormatter")]
    pub struct NSMeasurementFormatter;

    #[cfg(feature = "NSFormatter")]
    unsafe impl ClassType for NSMeasurementFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl NSCoding for NSMeasurementFormatter {}

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl NSCopying for NSMeasurementFormatter {}

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl CopyingHelper for NSMeasurementFormatter {
    type Result = Self;
}

#[cfg(feature = "NSFormatter")]
unsafe impl NSObjectProtocol for NSMeasurementFormatter {}

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl NSSecureCoding for NSMeasurementFormatter {}

extern_methods!(
    #[cfg(feature = "NSFormatter")]
    unsafe impl NSMeasurementFormatter {
        #[method(unitOptions)]
        pub unsafe fn unitOptions(&self) -> NSMeasurementFormatterUnitOptions;

        #[method(setUnitOptions:)]
        pub unsafe fn setUnitOptions(&self, unit_options: NSMeasurementFormatterUnitOptions);

        #[method(unitStyle)]
        pub unsafe fn unitStyle(&self) -> NSFormattingUnitStyle;

        #[method(setUnitStyle:)]
        pub unsafe fn setUnitStyle(&self, unit_style: NSFormattingUnitStyle);

        #[cfg(feature = "NSLocale")]
        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Retained<NSLocale>;

        #[cfg(feature = "NSLocale")]
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[cfg(feature = "NSNumberFormatter")]
        #[method_id(@__retain_semantics Other numberFormatter)]
        pub unsafe fn numberFormatter(&self) -> Retained<NSNumberFormatter>;

        #[cfg(feature = "NSNumberFormatter")]
        #[method(setNumberFormatter:)]
        pub unsafe fn setNumberFormatter(&self, number_formatter: Option<&NSNumberFormatter>);

        #[cfg(all(feature = "NSMeasurement", feature = "NSString"))]
        #[method_id(@__retain_semantics Other stringFromMeasurement:)]
        pub unsafe fn stringFromMeasurement(
            &self,
            measurement: &NSMeasurement,
        ) -> Retained<NSString>;

        #[cfg(all(feature = "NSString", feature = "NSUnit"))]
        #[method_id(@__retain_semantics Other stringFromUnit:)]
        pub unsafe fn stringFromUnit(&self, unit: &NSUnit) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSFormatter")]
    unsafe impl NSMeasurementFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

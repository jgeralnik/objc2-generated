//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/datadetection/ddmatch?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct DDMatch;
);

unsafe impl NSObjectProtocol for DDMatch {}

extern_methods!(
    unsafe impl DDMatch {
        #[method_id(@__retain_semantics Other matchedString)]
        pub unsafe fn matchedString(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl DDMatch {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/datadetection/ddmatchlink?language=objc)
    #[unsafe(super(DDMatch, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct DDMatchLink;
);

unsafe impl NSObjectProtocol for DDMatchLink {}

extern_methods!(
    unsafe impl DDMatchLink {
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Retained<NSURL>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DDMatch`
    unsafe impl DDMatchLink {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl DDMatchLink {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/datadetection/ddmatchphonenumber?language=objc)
    #[unsafe(super(DDMatch, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct DDMatchPhoneNumber;
);

unsafe impl NSObjectProtocol for DDMatchPhoneNumber {}

extern_methods!(
    unsafe impl DDMatchPhoneNumber {
        #[method_id(@__retain_semantics Other phoneNumber)]
        pub unsafe fn phoneNumber(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Option<Retained<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DDMatch`
    unsafe impl DDMatchPhoneNumber {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl DDMatchPhoneNumber {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/datadetection/ddmatchemailaddress?language=objc)
    #[unsafe(super(DDMatch, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct DDMatchEmailAddress;
);

unsafe impl NSObjectProtocol for DDMatchEmailAddress {}

extern_methods!(
    unsafe impl DDMatchEmailAddress {
        #[method_id(@__retain_semantics Other emailAddress)]
        pub unsafe fn emailAddress(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Option<Retained<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DDMatch`
    unsafe impl DDMatchEmailAddress {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl DDMatchEmailAddress {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/datadetection/ddmatchpostaladdress?language=objc)
    #[unsafe(super(DDMatch, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct DDMatchPostalAddress;
);

unsafe impl NSObjectProtocol for DDMatchPostalAddress {}

extern_methods!(
    unsafe impl DDMatchPostalAddress {
        #[method_id(@__retain_semantics Other street)]
        pub unsafe fn street(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other city)]
        pub unsafe fn city(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other state)]
        pub unsafe fn state(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other postalCode)]
        pub unsafe fn postalCode(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other country)]
        pub unsafe fn country(&self) -> Option<Retained<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DDMatch`
    unsafe impl DDMatchPostalAddress {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl DDMatchPostalAddress {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/datadetection/ddmatchcalendarevent?language=objc)
    #[unsafe(super(DDMatch, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct DDMatchCalendarEvent;
);

unsafe impl NSObjectProtocol for DDMatchCalendarEvent {}

extern_methods!(
    unsafe impl DDMatchCalendarEvent {
        #[method(isAllDay)]
        pub unsafe fn isAllDay(&self) -> bool;

        #[method_id(@__retain_semantics Other startDate)]
        pub unsafe fn startDate(&self) -> Option<Retained<NSDate>>;

        #[method_id(@__retain_semantics Other startTimeZone)]
        pub unsafe fn startTimeZone(&self) -> Option<Retained<NSTimeZone>>;

        #[method_id(@__retain_semantics Other endDate)]
        pub unsafe fn endDate(&self) -> Option<Retained<NSDate>>;

        #[method_id(@__retain_semantics Other endTimeZone)]
        pub unsafe fn endTimeZone(&self) -> Option<Retained<NSTimeZone>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DDMatch`
    unsafe impl DDMatchCalendarEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl DDMatchCalendarEvent {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/datadetection/ddmatchshipmenttrackingnumber?language=objc)
    #[unsafe(super(DDMatch, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct DDMatchShipmentTrackingNumber;
);

unsafe impl NSObjectProtocol for DDMatchShipmentTrackingNumber {}

extern_methods!(
    unsafe impl DDMatchShipmentTrackingNumber {
        #[method_id(@__retain_semantics Other carrier)]
        pub unsafe fn carrier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other trackingNumber)]
        pub unsafe fn trackingNumber(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DDMatch`
    unsafe impl DDMatchShipmentTrackingNumber {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl DDMatchShipmentTrackingNumber {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/datadetection/ddmatchflightnumber?language=objc)
    #[unsafe(super(DDMatch, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct DDMatchFlightNumber;
);

unsafe impl NSObjectProtocol for DDMatchFlightNumber {}

extern_methods!(
    unsafe impl DDMatchFlightNumber {
        #[method_id(@__retain_semantics Other airline)]
        pub unsafe fn airline(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other flightNumber)]
        pub unsafe fn flightNumber(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DDMatch`
    unsafe impl DDMatchFlightNumber {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl DDMatchFlightNumber {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/datadetection/ddmatchmoneyamount?language=objc)
    #[unsafe(super(DDMatch, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct DDMatchMoneyAmount;
);

unsafe impl NSObjectProtocol for DDMatchMoneyAmount {}

extern_methods!(
    unsafe impl DDMatchMoneyAmount {
        #[method_id(@__retain_semantics Other currency)]
        pub unsafe fn currency(&self) -> Retained<NSString>;

        #[method(amount)]
        pub unsafe fn amount(&self) -> c_double;
    }
);

extern_methods!(
    /// Methods declared on superclass `DDMatch`
    unsafe impl DDMatchMoneyAmount {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl DDMatchMoneyAmount {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/eventkit/ekcalendaritem?language=objc)
    #[unsafe(super(EKObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "EKObject")]
    pub struct EKCalendarItem;
);

#[cfg(feature = "EKObject")]
unsafe impl NSObjectProtocol for EKCalendarItem {}

extern_methods!(
    #[cfg(feature = "EKObject")]
    unsafe impl EKCalendarItem {
        #[method_id(@__retain_semantics Other UUID)]
        pub unsafe fn UUID(&self) -> Retained<NSString>;

        #[cfg(feature = "EKCalendar")]
        #[method_id(@__retain_semantics Other calendar)]
        pub unsafe fn calendar(&self) -> Option<Retained<EKCalendar>>;

        #[cfg(feature = "EKCalendar")]
        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: Option<&EKCalendar>);

        #[method_id(@__retain_semantics Other calendarItemIdentifier)]
        pub unsafe fn calendarItemIdentifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other calendarItemExternalIdentifier)]
        pub unsafe fn calendarItemExternalIdentifier(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[method_id(@__retain_semantics Other location)]
        pub unsafe fn location(&self) -> Option<Retained<NSString>>;

        #[method(setLocation:)]
        pub unsafe fn setLocation(&self, location: Option<&NSString>);

        #[method_id(@__retain_semantics Other notes)]
        pub unsafe fn notes(&self) -> Option<Retained<NSString>>;

        #[method(setNotes:)]
        pub unsafe fn setNotes(&self, notes: Option<&NSString>);

        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Retained<NSURL>>;

        #[method(setURL:)]
        pub unsafe fn setURL(&self, url: Option<&NSURL>);

        #[method_id(@__retain_semantics Other lastModifiedDate)]
        pub unsafe fn lastModifiedDate(&self) -> Option<Retained<NSDate>>;

        #[method_id(@__retain_semantics Other creationDate)]
        pub unsafe fn creationDate(&self) -> Option<Retained<NSDate>>;

        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Option<Retained<NSTimeZone>>;

        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, time_zone: Option<&NSTimeZone>);

        #[method(hasAlarms)]
        pub unsafe fn hasAlarms(&self) -> bool;

        #[method(hasRecurrenceRules)]
        pub unsafe fn hasRecurrenceRules(&self) -> bool;

        #[method(hasAttendees)]
        pub unsafe fn hasAttendees(&self) -> bool;

        #[method(hasNotes)]
        pub unsafe fn hasNotes(&self) -> bool;

        #[cfg(feature = "EKParticipant")]
        #[method_id(@__retain_semantics Other attendees)]
        pub unsafe fn attendees(&self) -> Option<Retained<NSArray<EKParticipant>>>;

        #[cfg(feature = "EKAlarm")]
        #[method_id(@__retain_semantics Other alarms)]
        pub unsafe fn alarms(&self) -> Option<Retained<NSArray<EKAlarm>>>;

        #[cfg(feature = "EKAlarm")]
        #[method(setAlarms:)]
        pub unsafe fn setAlarms(&self, alarms: Option<&NSArray<EKAlarm>>);

        #[cfg(feature = "EKAlarm")]
        #[method(addAlarm:)]
        pub unsafe fn addAlarm(&self, alarm: &EKAlarm);

        #[cfg(feature = "EKAlarm")]
        #[method(removeAlarm:)]
        pub unsafe fn removeAlarm(&self, alarm: &EKAlarm);

        #[cfg(feature = "EKRecurrenceRule")]
        #[method_id(@__retain_semantics Other recurrenceRules)]
        pub unsafe fn recurrenceRules(&self) -> Option<Retained<NSArray<EKRecurrenceRule>>>;

        #[cfg(feature = "EKRecurrenceRule")]
        #[method(setRecurrenceRules:)]
        pub unsafe fn setRecurrenceRules(
            &self,
            recurrence_rules: Option<&NSArray<EKRecurrenceRule>>,
        );

        #[cfg(feature = "EKRecurrenceRule")]
        #[method(addRecurrenceRule:)]
        pub unsafe fn addRecurrenceRule(&self, rule: &EKRecurrenceRule);

        #[cfg(feature = "EKRecurrenceRule")]
        #[method(removeRecurrenceRule:)]
        pub unsafe fn removeRecurrenceRule(&self, rule: &EKRecurrenceRule);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "EKObject")]
    unsafe impl EKCalendarItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/eventkit/ekspan?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EKSpan(pub NSInteger);
impl EKSpan {
    #[doc(alias = "EKSpanThisEvent")]
    pub const ThisEvent: Self = Self(0);
    #[doc(alias = "EKSpanFutureEvents")]
    pub const FutureEvents: Self = Self(1);
}

unsafe impl Encode for EKSpan {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for EKSpan {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/eventkit/ekeventsearchcallback?language=objc)
#[cfg(all(
    feature = "EKCalendarItem",
    feature = "EKEvent",
    feature = "EKObject",
    feature = "block2"
))]
pub type EKEventSearchCallback = *mut block2::Block<dyn Fn(NonNull<EKEvent>, NonNull<Bool>)>;

/// [Apple's documentation](https://developer.apple.com/documentation/eventkit/ekeventstorerequestaccesscompletionhandler?language=objc)
#[cfg(feature = "block2")]
pub type EKEventStoreRequestAccessCompletionHandler =
    *mut block2::Block<dyn Fn(Bool, *mut NSError)>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/eventkit/ekeventstore?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct EKEventStore;
);

unsafe impl NSObjectProtocol for EKEventStore {}

extern_methods!(
    unsafe impl EKEventStore {
        #[cfg(feature = "EKTypes")]
        #[method(authorizationStatusForEntityType:)]
        pub unsafe fn authorizationStatusForEntityType(
            entity_type: EKEntityType,
        ) -> EKAuthorizationStatus;

        #[cfg(feature = "EKTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithAccessToEntityTypes:)]
        pub unsafe fn initWithAccessToEntityTypes(
            this: Allocated<Self>,
            entity_types: EKEntityMask,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(feature = "EKObject", feature = "EKSource"))]
        #[method_id(@__retain_semantics Init initWithSources:)]
        pub unsafe fn initWithSources(
            this: Allocated<Self>,
            sources: &NSArray<EKSource>,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method(requestFullAccessToEventsWithCompletion:)]
        pub unsafe fn requestFullAccessToEventsWithCompletion(
            &self,
            completion: EKEventStoreRequestAccessCompletionHandler,
        );

        #[cfg(feature = "block2")]
        #[method(requestWriteOnlyAccessToEventsWithCompletion:)]
        pub unsafe fn requestWriteOnlyAccessToEventsWithCompletion(
            &self,
            completion: EKEventStoreRequestAccessCompletionHandler,
        );

        #[cfg(feature = "block2")]
        #[method(requestFullAccessToRemindersWithCompletion:)]
        pub unsafe fn requestFullAccessToRemindersWithCompletion(
            &self,
            completion: EKEventStoreRequestAccessCompletionHandler,
        );

        #[cfg(all(feature = "EKTypes", feature = "block2"))]
        #[deprecated = "Use -requestFullAccessToEventsWithCompletion:, -requestWriteOnlyAccessToEventsWithCompletion:, or -requestFullAccessToRemindersWithCompletion:"]
        #[method(requestAccessToEntityType:completion:)]
        pub unsafe fn requestAccessToEntityType_completion(
            &self,
            entity_type: EKEntityType,
            completion: EKEventStoreRequestAccessCompletionHandler,
        );

        #[method_id(@__retain_semantics Other eventStoreIdentifier)]
        pub unsafe fn eventStoreIdentifier(&self) -> Retained<NSString>;

        #[cfg(all(feature = "EKObject", feature = "EKSource"))]
        #[method_id(@__retain_semantics Other delegateSources)]
        pub unsafe fn delegateSources(&self) -> Retained<NSArray<EKSource>>;

        #[cfg(all(feature = "EKObject", feature = "EKSource"))]
        #[method_id(@__retain_semantics Other sources)]
        pub unsafe fn sources(&self) -> Retained<NSArray<EKSource>>;

        #[cfg(all(feature = "EKObject", feature = "EKSource"))]
        #[method_id(@__retain_semantics Other sourceWithIdentifier:)]
        pub unsafe fn sourceWithIdentifier(
            &self,
            identifier: &NSString,
        ) -> Option<Retained<EKSource>>;

        #[cfg(all(feature = "EKCalendar", feature = "EKObject"))]
        #[method_id(@__retain_semantics Other calendars)]
        pub unsafe fn calendars(&self) -> Retained<NSArray<EKCalendar>>;

        #[cfg(all(feature = "EKCalendar", feature = "EKObject", feature = "EKTypes"))]
        #[method_id(@__retain_semantics Other calendarsForEntityType:)]
        pub unsafe fn calendarsForEntityType(
            &self,
            entity_type: EKEntityType,
        ) -> Retained<NSArray<EKCalendar>>;

        #[cfg(all(feature = "EKCalendar", feature = "EKObject"))]
        #[method_id(@__retain_semantics Other defaultCalendarForNewEvents)]
        pub unsafe fn defaultCalendarForNewEvents(&self) -> Option<Retained<EKCalendar>>;

        #[cfg(all(feature = "EKCalendar", feature = "EKObject"))]
        #[method_id(@__retain_semantics Other defaultCalendarForNewReminders)]
        pub unsafe fn defaultCalendarForNewReminders(&self) -> Option<Retained<EKCalendar>>;

        #[cfg(all(feature = "EKCalendar", feature = "EKObject"))]
        #[method_id(@__retain_semantics Other calendarWithIdentifier:)]
        pub unsafe fn calendarWithIdentifier(
            &self,
            identifier: &NSString,
        ) -> Option<Retained<EKCalendar>>;

        #[cfg(all(feature = "EKCalendar", feature = "EKObject"))]
        #[method(saveCalendar:commit:error:_)]
        pub unsafe fn saveCalendar_commit_error(
            &self,
            calendar: &EKCalendar,
            commit: bool,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(all(feature = "EKCalendar", feature = "EKObject"))]
        #[method(removeCalendar:commit:error:_)]
        pub unsafe fn removeCalendar_commit_error(
            &self,
            calendar: &EKCalendar,
            commit: bool,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(all(feature = "EKCalendarItem", feature = "EKObject"))]
        #[method_id(@__retain_semantics Other calendarItemWithIdentifier:)]
        pub unsafe fn calendarItemWithIdentifier(
            &self,
            identifier: &NSString,
        ) -> Option<Retained<EKCalendarItem>>;

        #[cfg(all(feature = "EKCalendarItem", feature = "EKObject"))]
        #[method_id(@__retain_semantics Other calendarItemsWithExternalIdentifier:)]
        pub unsafe fn calendarItemsWithExternalIdentifier(
            &self,
            external_identifier: &NSString,
        ) -> Retained<NSArray<EKCalendarItem>>;

        #[cfg(all(feature = "EKCalendarItem", feature = "EKEvent", feature = "EKObject"))]
        #[method(saveEvent:span:error:_)]
        pub unsafe fn saveEvent_span_error(
            &self,
            event: &EKEvent,
            span: EKSpan,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(all(feature = "EKCalendarItem", feature = "EKEvent", feature = "EKObject"))]
        #[method(removeEvent:span:error:_)]
        pub unsafe fn removeEvent_span_error(
            &self,
            event: &EKEvent,
            span: EKSpan,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(all(feature = "EKCalendarItem", feature = "EKEvent", feature = "EKObject"))]
        #[method(saveEvent:span:commit:error:_)]
        pub unsafe fn saveEvent_span_commit_error(
            &self,
            event: &EKEvent,
            span: EKSpan,
            commit: bool,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(all(feature = "EKCalendarItem", feature = "EKEvent", feature = "EKObject"))]
        #[method(removeEvent:span:commit:error:_)]
        pub unsafe fn removeEvent_span_commit_error(
            &self,
            event: &EKEvent,
            span: EKSpan,
            commit: bool,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(all(feature = "EKCalendarItem", feature = "EKEvent", feature = "EKObject"))]
        #[method_id(@__retain_semantics Other eventWithIdentifier:)]
        pub unsafe fn eventWithIdentifier(
            &self,
            identifier: &NSString,
        ) -> Option<Retained<EKEvent>>;

        #[cfg(all(feature = "EKCalendarItem", feature = "EKEvent", feature = "EKObject"))]
        #[method_id(@__retain_semantics Other eventsMatchingPredicate:)]
        pub unsafe fn eventsMatchingPredicate(
            &self,
            predicate: &NSPredicate,
        ) -> Retained<NSArray<EKEvent>>;

        #[cfg(all(
            feature = "EKCalendarItem",
            feature = "EKEvent",
            feature = "EKObject",
            feature = "block2"
        ))]
        #[method(enumerateEventsMatchingPredicate:usingBlock:)]
        pub unsafe fn enumerateEventsMatchingPredicate_usingBlock(
            &self,
            predicate: &NSPredicate,
            block: EKEventSearchCallback,
        );

        #[cfg(all(feature = "EKCalendar", feature = "EKObject"))]
        #[method_id(@__retain_semantics Other predicateForEventsWithStartDate:endDate:calendars:)]
        pub unsafe fn predicateForEventsWithStartDate_endDate_calendars(
            &self,
            start_date: &NSDate,
            end_date: &NSDate,
            calendars: Option<&NSArray<EKCalendar>>,
        ) -> Retained<NSPredicate>;

        #[cfg(all(
            feature = "EKCalendarItem",
            feature = "EKObject",
            feature = "EKReminder"
        ))]
        #[method(saveReminder:commit:error:_)]
        pub unsafe fn saveReminder_commit_error(
            &self,
            reminder: &EKReminder,
            commit: bool,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(all(
            feature = "EKCalendarItem",
            feature = "EKObject",
            feature = "EKReminder"
        ))]
        #[method(removeReminder:commit:error:_)]
        pub unsafe fn removeReminder_commit_error(
            &self,
            reminder: &EKReminder,
            commit: bool,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(all(
            feature = "EKCalendarItem",
            feature = "EKObject",
            feature = "EKReminder",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Other fetchRemindersMatchingPredicate:completion:)]
        pub unsafe fn fetchRemindersMatchingPredicate_completion(
            &self,
            predicate: &NSPredicate,
            completion: &block2::Block<dyn Fn(*mut NSArray<EKReminder>)>,
        ) -> Retained<AnyObject>;

        #[method(cancelFetchRequest:)]
        pub unsafe fn cancelFetchRequest(&self, fetch_identifier: &AnyObject);

        #[cfg(all(feature = "EKCalendar", feature = "EKObject"))]
        #[method_id(@__retain_semantics Other predicateForRemindersInCalendars:)]
        pub unsafe fn predicateForRemindersInCalendars(
            &self,
            calendars: Option<&NSArray<EKCalendar>>,
        ) -> Retained<NSPredicate>;

        #[cfg(all(feature = "EKCalendar", feature = "EKObject"))]
        #[method_id(@__retain_semantics Other predicateForIncompleteRemindersWithDueDateStarting:ending:calendars:)]
        pub unsafe fn predicateForIncompleteRemindersWithDueDateStarting_ending_calendars(
            &self,
            start_date: Option<&NSDate>,
            end_date: Option<&NSDate>,
            calendars: Option<&NSArray<EKCalendar>>,
        ) -> Retained<NSPredicate>;

        #[cfg(all(feature = "EKCalendar", feature = "EKObject"))]
        #[method_id(@__retain_semantics Other predicateForCompletedRemindersWithCompletionDateStarting:ending:calendars:)]
        pub unsafe fn predicateForCompletedRemindersWithCompletionDateStarting_ending_calendars(
            &self,
            start_date: Option<&NSDate>,
            end_date: Option<&NSDate>,
            calendars: Option<&NSArray<EKCalendar>>,
        ) -> Retained<NSPredicate>;

        #[method(commit:_)]
        pub unsafe fn commit(&self) -> Result<(), Retained<NSError>>;

        #[method(reset)]
        pub unsafe fn reset(&self);

        #[method(refreshSourcesIfNecessary)]
        pub unsafe fn refreshSourcesIfNecessary(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl EKEventStore {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/eventkit/ekeventstorechangednotification?language=objc)
    pub static EKEventStoreChangedNotification: &'static NSString;
}

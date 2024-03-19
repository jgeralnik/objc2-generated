//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::EventKit::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static EKErrorDomain: Option<&'static NSString>;
}

ns_enum!(
    #[underlying(NSInteger)]
    pub enum EKErrorCode {
        EKErrorEventNotMutable = 0,
        EKErrorNoCalendar = 1,
        EKErrorNoStartDate = 2,
        EKErrorNoEndDate = 3,
        EKErrorDatesInverted = 4,
        EKErrorInternalFailure = 5,
        EKErrorCalendarReadOnly = 6,
        EKErrorDurationGreaterThanRecurrence = 7,
        EKErrorAlarmGreaterThanRecurrence = 8,
        EKErrorStartDateTooFarInFuture = 9,
        EKErrorStartDateCollidesWithOtherOccurrence = 10,
        EKErrorObjectBelongsToDifferentStore = 11,
        EKErrorInvitesCannotBeMoved = 12,
        EKErrorInvalidSpan = 13,
        EKErrorCalendarHasNoSource = 14,
        EKErrorCalendarSourceCannotBeModified = 15,
        EKErrorCalendarIsImmutable = 16,
        EKErrorSourceDoesNotAllowCalendarAddDelete = 17,
        EKErrorRecurringReminderRequiresDueDate = 18,
        EKErrorStructuredLocationsNotSupported = 19,
        EKErrorReminderLocationsNotSupported = 20,
        EKErrorAlarmProximityNotSupported = 21,
        EKErrorCalendarDoesNotAllowEvents = 22,
        EKErrorCalendarDoesNotAllowReminders = 23,
        EKErrorSourceDoesNotAllowReminders = 24,
        EKErrorSourceDoesNotAllowEvents = 25,
        EKErrorPriorityIsInvalid = 26,
        EKErrorInvalidEntityType = 27,
        EKErrorProcedureAlarmsNotMutable = 28,
        EKErrorEventStoreNotAuthorized = 29,
        EKErrorOSNotSupported = 30,
        EKErrorInvalidInviteReplyCalendar = 31,
        EKErrorNotificationsCollectionFlagNotSet = 32,
        EKErrorSourceMismatch = 33,
        EKErrorNotificationCollectionMismatch = 34,
        EKErrorNotificationSavedWithoutCollection = 35,
        EKErrorReminderAlarmContainsEmailOrUrl = 36,
        EKErrorLast = 37,
    }
);

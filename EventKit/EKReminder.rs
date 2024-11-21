//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(EKCalendarItem, EKObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "EKCalendarItem", feature = "EKObject"))]
    pub struct EKReminder;
);

#[cfg(all(feature = "EKCalendarItem", feature = "EKObject"))]
unsafe impl NSObjectProtocol for EKReminder {}

extern_methods!(
    #[cfg(all(feature = "EKCalendarItem", feature = "EKObject"))]
    unsafe impl EKReminder {
        #[cfg(feature = "EKEventStore")]
        #[method_id(@__retain_semantics Other reminderWithEventStore:)]
        pub unsafe fn reminderWithEventStore(event_store: &EKEventStore) -> Retained<EKReminder>;

        #[method_id(@__retain_semantics Other startDateComponents)]
        pub unsafe fn startDateComponents(&self) -> Option<Retained<NSDateComponents>>;

        #[method(setStartDateComponents:)]
        pub unsafe fn setStartDateComponents(
            &self,
            start_date_components: Option<&NSDateComponents>,
        );

        #[method_id(@__retain_semantics Other dueDateComponents)]
        pub unsafe fn dueDateComponents(&self) -> Option<Retained<NSDateComponents>>;

        #[method(setDueDateComponents:)]
        pub unsafe fn setDueDateComponents(&self, due_date_components: Option<&NSDateComponents>);

        #[method(isCompleted)]
        pub unsafe fn isCompleted(&self) -> bool;

        #[method(setCompleted:)]
        pub unsafe fn setCompleted(&self, completed: bool);

        #[method_id(@__retain_semantics Other completionDate)]
        pub unsafe fn completionDate(&self) -> Option<Retained<NSDate>>;

        #[method(setCompletionDate:)]
        pub unsafe fn setCompletionDate(&self, completion_date: Option<&NSDate>);

        #[method(priority)]
        pub unsafe fn priority(&self) -> NSUInteger;

        #[method(setPriority:)]
        pub unsafe fn setPriority(&self, priority: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "EKCalendarItem", feature = "EKObject"))]
    unsafe impl EKReminder {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

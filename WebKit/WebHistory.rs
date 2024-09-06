//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static WebHistoryItemsAddedNotification: Option<&'static NSString>;
}

extern "C" {
    pub static WebHistoryItemsRemovedNotification: Option<&'static NSString>;
}

extern "C" {
    pub static WebHistoryAllItemsRemovedNotification: Option<&'static NSString>;
}

extern "C" {
    pub static WebHistoryLoadedNotification: Option<&'static NSString>;
}

extern "C" {
    pub static WebHistorySavedNotification: Option<&'static NSString>;
}

extern "C" {
    pub static WebHistoryItemsKey: Option<&'static NSString>;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct WebHistory;

    unsafe impl ClassType for WebHistory {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for WebHistory {}

extern_methods!(
    unsafe impl WebHistory {
        #[deprecated]
        #[method_id(@__retain_semantics Other optionalSharedHistory)]
        pub unsafe fn optionalSharedHistory() -> Option<Retained<WebHistory>>;

        #[deprecated]
        #[method(setOptionalSharedHistory:)]
        pub unsafe fn setOptionalSharedHistory(history: Option<&WebHistory>);

        #[deprecated]
        #[method(addItems:)]
        pub unsafe fn addItems(&self, new_items: Option<&NSArray>);

        #[deprecated]
        #[method(removeItems:)]
        pub unsafe fn removeItems(&self, items: Option<&NSArray>);

        #[deprecated]
        #[method(removeAllItems)]
        pub unsafe fn removeAllItems(&self);

        #[deprecated]
        #[method_id(@__retain_semantics Other orderedLastVisitedDays)]
        pub unsafe fn orderedLastVisitedDays(&self) -> Retained<NSArray>;

        #[deprecated]
        #[method_id(@__retain_semantics Other orderedItemsLastVisitedOnDay:)]
        pub unsafe fn orderedItemsLastVisitedOnDay(
            &self,
            calendar_date: Option<&NSCalendarDate>,
        ) -> Option<Retained<NSArray>>;

        #[cfg(feature = "WebHistoryItem")]
        #[deprecated]
        #[method_id(@__retain_semantics Other itemForURL:)]
        pub unsafe fn itemForURL(&self, url: Option<&NSURL>) -> Option<Retained<WebHistoryItem>>;

        #[deprecated]
        #[method(historyItemLimit)]
        pub unsafe fn historyItemLimit(&self) -> c_int;

        #[deprecated]
        #[method(setHistoryItemLimit:)]
        pub unsafe fn setHistoryItemLimit(&self, history_item_limit: c_int);

        #[deprecated]
        #[method(historyAgeInDaysLimit)]
        pub unsafe fn historyAgeInDaysLimit(&self) -> c_int;

        #[deprecated]
        #[method(setHistoryAgeInDaysLimit:)]
        pub unsafe fn setHistoryAgeInDaysLimit(&self, history_age_in_days_limit: c_int);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WebHistory {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

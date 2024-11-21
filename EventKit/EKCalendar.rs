//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(EKObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "EKObject")]
    pub struct EKCalendar;
);

#[cfg(feature = "EKObject")]
unsafe impl NSObjectProtocol for EKCalendar {}

extern_methods!(
    #[cfg(feature = "EKObject")]
    unsafe impl EKCalendar {
        #[cfg(feature = "EKEventStore")]
        #[method_id(@__retain_semantics Other calendarWithEventStore:)]
        pub unsafe fn calendarWithEventStore(event_store: &EKEventStore) -> Retained<EKCalendar>;

        #[cfg(all(feature = "EKEventStore", feature = "EKTypes"))]
        #[method_id(@__retain_semantics Other calendarForEntityType:eventStore:)]
        pub unsafe fn calendarForEntityType_eventStore(
            entity_type: EKEntityType,
            event_store: &EKEventStore,
        ) -> Retained<EKCalendar>;

        #[cfg(feature = "EKSource")]
        #[method_id(@__retain_semantics Other source)]
        pub unsafe fn source(&self) -> Option<Retained<EKSource>>;

        #[cfg(feature = "EKSource")]
        #[method(setSource:)]
        pub unsafe fn setSource(&self, source: Option<&EKSource>);

        #[method_id(@__retain_semantics Other calendarIdentifier)]
        pub unsafe fn calendarIdentifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[cfg(feature = "EKTypes")]
        #[method(type)]
        pub unsafe fn r#type(&self) -> EKCalendarType;

        #[method(allowsContentModifications)]
        pub unsafe fn allowsContentModifications(&self) -> bool;

        #[method(isSubscribed)]
        pub unsafe fn isSubscribed(&self) -> bool;

        #[method(isImmutable)]
        pub unsafe fn isImmutable(&self) -> bool;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Other color)]
        pub unsafe fn color(&self) -> Retained<NSColor>;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: Option<&NSColor>);

        #[cfg(feature = "EKTypes")]
        #[method(supportedEventAvailabilities)]
        pub unsafe fn supportedEventAvailabilities(&self) -> EKCalendarEventAvailabilityMask;

        #[cfg(feature = "EKTypes")]
        #[method(allowedEntityTypes)]
        pub unsafe fn allowedEntityTypes(&self) -> EKEntityMask;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "EKObject")]
    unsafe impl EKCalendar {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

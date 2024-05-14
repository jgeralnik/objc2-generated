//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CMPedometerData;

    unsafe impl ClassType for CMPedometerData {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CMPedometerData {}

unsafe impl NSCopying for CMPedometerData {}

unsafe impl NSObjectProtocol for CMPedometerData {}

unsafe impl NSSecureCoding for CMPedometerData {}

extern_methods!(
    unsafe impl CMPedometerData {
        #[method_id(@__retain_semantics Other startDate)]
        pub unsafe fn startDate(&self) -> Id<NSDate>;

        #[method_id(@__retain_semantics Other endDate)]
        pub unsafe fn endDate(&self) -> Id<NSDate>;

        #[method_id(@__retain_semantics Other numberOfSteps)]
        pub unsafe fn numberOfSteps(&self) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Other distance)]
        pub unsafe fn distance(&self) -> Option<Id<NSNumber>>;

        #[method_id(@__retain_semantics Other floorsAscended)]
        pub unsafe fn floorsAscended(&self) -> Option<Id<NSNumber>>;

        #[method_id(@__retain_semantics Other floorsDescended)]
        pub unsafe fn floorsDescended(&self) -> Option<Id<NSNumber>>;

        #[method_id(@__retain_semantics Other currentPace)]
        pub unsafe fn currentPace(&self) -> Option<Id<NSNumber>>;

        #[method_id(@__retain_semantics Other currentCadence)]
        pub unsafe fn currentCadence(&self) -> Option<Id<NSNumber>>;

        #[method_id(@__retain_semantics Other averageActivePace)]
        pub unsafe fn averageActivePace(&self) -> Option<Id<NSNumber>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CMPedometerData {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CMPedometerEventType(pub NSInteger);
impl CMPedometerEventType {
    #[doc(alias = "CMPedometerEventTypePause")]
    pub const Pause: Self = Self(0);
    #[doc(alias = "CMPedometerEventTypeResume")]
    pub const Resume: Self = Self(1);
}

unsafe impl Encode for CMPedometerEventType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CMPedometerEventType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CMPedometerEvent;

    unsafe impl ClassType for CMPedometerEvent {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CMPedometerEvent {}

unsafe impl NSCopying for CMPedometerEvent {}

unsafe impl NSObjectProtocol for CMPedometerEvent {}

unsafe impl NSSecureCoding for CMPedometerEvent {}

extern_methods!(
    unsafe impl CMPedometerEvent {
        #[method_id(@__retain_semantics Other date)]
        pub unsafe fn date(&self) -> Id<NSDate>;

        #[method(type)]
        pub unsafe fn r#type(&self) -> CMPedometerEventType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CMPedometerEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

#[cfg(feature = "block2")]
pub type CMPedometerHandler = *mut block2::Block<dyn Fn(*mut CMPedometerData, *mut NSError)>;

#[cfg(feature = "block2")]
pub type CMPedometerEventHandler = *mut block2::Block<dyn Fn(*mut CMPedometerEvent, *mut NSError)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CMPedometer;

    unsafe impl ClassType for CMPedometer {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for CMPedometer {}

extern_methods!(
    unsafe impl CMPedometer {
        #[method(isStepCountingAvailable)]
        pub unsafe fn isStepCountingAvailable() -> bool;

        #[method(isDistanceAvailable)]
        pub unsafe fn isDistanceAvailable() -> bool;

        #[method(isFloorCountingAvailable)]
        pub unsafe fn isFloorCountingAvailable() -> bool;

        #[method(isPaceAvailable)]
        pub unsafe fn isPaceAvailable() -> bool;

        #[method(isCadenceAvailable)]
        pub unsafe fn isCadenceAvailable() -> bool;

        #[method(isPedometerEventTrackingAvailable)]
        pub unsafe fn isPedometerEventTrackingAvailable() -> bool;

        #[cfg(feature = "CMAuthorization")]
        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus() -> CMAuthorizationStatus;

        #[cfg(feature = "block2")]
        #[method(queryPedometerDataFromDate:toDate:withHandler:)]
        pub unsafe fn queryPedometerDataFromDate_toDate_withHandler(
            &self,
            start: &NSDate,
            end: &NSDate,
            handler: CMPedometerHandler,
        );

        #[cfg(feature = "block2")]
        #[method(startPedometerUpdatesFromDate:withHandler:)]
        pub unsafe fn startPedometerUpdatesFromDate_withHandler(
            &self,
            start: &NSDate,
            handler: CMPedometerHandler,
        );

        #[method(stopPedometerUpdates)]
        pub unsafe fn stopPedometerUpdates(&self);

        #[cfg(feature = "block2")]
        #[method(startPedometerEventUpdatesWithHandler:)]
        pub unsafe fn startPedometerEventUpdatesWithHandler(
            &self,
            handler: CMPedometerEventHandler,
        );

        #[method(stopPedometerEventUpdates)]
        pub unsafe fn stopPedometerEventUpdates(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CMPedometer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

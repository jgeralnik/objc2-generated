//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[cfg(all(feature = "CMAltitude", feature = "CMLogItem", feature = "block2"))]
pub type CMAltitudeHandler = *mut block2::Block<dyn Fn(*mut CMAltitudeData, *mut NSError)>;

#[cfg(all(
    feature = "CMAbsoluteAltitude",
    feature = "CMLogItem",
    feature = "block2"
))]
pub type CMAbsoluteAltitudeHandler =
    *mut block2::Block<dyn Fn(*mut CMAbsoluteAltitudeData, *mut NSError)>;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CMAltimeter;
);

unsafe impl NSObjectProtocol for CMAltimeter {}

extern_methods!(
    unsafe impl CMAltimeter {
        #[method(isRelativeAltitudeAvailable)]
        pub unsafe fn isRelativeAltitudeAvailable() -> bool;

        #[cfg(feature = "CMAuthorization")]
        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus() -> CMAuthorizationStatus;

        #[cfg(all(feature = "CMAltitude", feature = "CMLogItem", feature = "block2"))]
        #[method(startRelativeAltitudeUpdatesToQueue:withHandler:)]
        pub unsafe fn startRelativeAltitudeUpdatesToQueue_withHandler(
            &self,
            queue: &NSOperationQueue,
            handler: CMAltitudeHandler,
        );

        #[method(stopRelativeAltitudeUpdates)]
        pub unsafe fn stopRelativeAltitudeUpdates(&self);

        #[method(isAbsoluteAltitudeAvailable)]
        pub unsafe fn isAbsoluteAltitudeAvailable() -> bool;

        #[cfg(all(
            feature = "CMAbsoluteAltitude",
            feature = "CMLogItem",
            feature = "block2"
        ))]
        #[method(startAbsoluteAltitudeUpdatesToQueue:withHandler:)]
        pub unsafe fn startAbsoluteAltitudeUpdatesToQueue_withHandler(
            &self,
            queue: &NSOperationQueue,
            handler: CMAbsoluteAltitudeHandler,
        );

        #[method(stopAbsoluteAltitudeUpdates)]
        pub unsafe fn stopAbsoluteAltitudeUpdates(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CMAltimeter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

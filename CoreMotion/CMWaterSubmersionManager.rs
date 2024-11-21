//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait CMWaterSubmersionManagerDelegate: NSObjectProtocol {
        #[cfg(feature = "CMWaterSubmersionData")]
        #[method(manager:didUpdateEvent:)]
        unsafe fn manager_didUpdateEvent(
            &self,
            manager: &CMWaterSubmersionManager,
            event: &CMWaterSubmersionEvent,
        );

        #[cfg(feature = "CMWaterSubmersionData")]
        #[method(manager:didUpdateMeasurement:)]
        unsafe fn manager_didUpdateMeasurement(
            &self,
            manager: &CMWaterSubmersionManager,
            measurement: &CMWaterSubmersionMeasurement,
        );

        #[cfg(feature = "CMWaterSubmersionData")]
        #[method(manager:didUpdateTemperature:)]
        unsafe fn manager_didUpdateTemperature(
            &self,
            manager: &CMWaterSubmersionManager,
            measurement: &CMWaterTemperature,
        );

        #[method(manager:errorOccurred:)]
        unsafe fn manager_errorOccurred(&self, manager: &CMWaterSubmersionManager, error: &NSError);
    }

    unsafe impl ProtocolType for dyn CMWaterSubmersionManagerDelegate {}
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CMWaterSubmersionManager;
);

unsafe impl NSObjectProtocol for CMWaterSubmersionManager {}

extern_methods!(
    unsafe impl CMWaterSubmersionManager {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn CMWaterSubmersionManagerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn CMWaterSubmersionManagerDelegate>>,
        );

        #[method(waterSubmersionAvailable)]
        pub unsafe fn waterSubmersionAvailable() -> bool;

        #[cfg(feature = "CMAuthorization")]
        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus() -> CMAuthorizationStatus;

        #[method_id(@__retain_semantics Other maximumDepth)]
        pub unsafe fn maximumDepth(&self) -> Option<Retained<NSMeasurement<NSUnitLength>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CMWaterSubmersionManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

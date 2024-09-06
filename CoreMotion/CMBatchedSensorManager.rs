//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CMBatchedSensorManager;

    unsafe impl ClassType for CMBatchedSensorManager {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for CMBatchedSensorManager {}

extern_methods!(
    unsafe impl CMBatchedSensorManager {
        #[cfg(feature = "CMAuthorization")]
        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus() -> CMAuthorizationStatus;

        #[method(isAccelerometerSupported)]
        pub unsafe fn isAccelerometerSupported() -> bool;

        #[method(isAccelerometerActive)]
        pub unsafe fn isAccelerometerActive(&self) -> bool;

        #[method(accelerometerDataFrequency)]
        pub unsafe fn accelerometerDataFrequency(&self) -> NSInteger;

        #[cfg(all(feature = "CMAccelerometer", feature = "CMLogItem"))]
        #[method_id(@__retain_semantics Other accelerometerBatch)]
        pub unsafe fn accelerometerBatch(&self) -> Option<Retained<NSArray<CMAccelerometerData>>>;

        #[method(startAccelerometerUpdates)]
        pub unsafe fn startAccelerometerUpdates(&self);

        #[cfg(all(feature = "CMAccelerometer", feature = "CMLogItem", feature = "block2"))]
        #[method(startAccelerometerUpdatesWithHandler:)]
        pub unsafe fn startAccelerometerUpdatesWithHandler(
            &self,
            handler: &block2::Block<dyn Fn(*mut NSArray<CMAccelerometerData>, *mut NSError)>,
        );

        #[method(stopAccelerometerUpdates)]
        pub unsafe fn stopAccelerometerUpdates(&self);

        #[method(isDeviceMotionSupported)]
        pub unsafe fn isDeviceMotionSupported() -> bool;

        #[method(deviceMotionDataFrequency)]
        pub unsafe fn deviceMotionDataFrequency(&self) -> NSInteger;

        #[method(isDeviceMotionActive)]
        pub unsafe fn isDeviceMotionActive(&self) -> bool;

        #[cfg(all(feature = "CMDeviceMotion", feature = "CMLogItem"))]
        #[method_id(@__retain_semantics Other deviceMotionBatch)]
        pub unsafe fn deviceMotionBatch(&self) -> Option<Retained<NSArray<CMDeviceMotion>>>;

        #[method(startDeviceMotionUpdates)]
        pub unsafe fn startDeviceMotionUpdates(&self);

        #[cfg(all(feature = "CMDeviceMotion", feature = "CMLogItem", feature = "block2"))]
        #[method(startDeviceMotionUpdatesWithHandler:)]
        pub unsafe fn startDeviceMotionUpdatesWithHandler(
            &self,
            handler: &block2::Block<dyn Fn(*mut NSArray<CMDeviceMotion>, *mut NSError)>,
        );

        #[method(stopDeviceMotionUpdates)]
        pub unsafe fn stopDeviceMotionUpdates(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CMBatchedSensorManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

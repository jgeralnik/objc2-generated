//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

pub type UIAccelerationValue = c_double;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "UIAcceleration has been replaced by the CoreMotion framework"]
    pub struct UIAcceleration;

    unsafe impl ClassType for UIAcceleration {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UIAcceleration {}

extern_methods!(
    unsafe impl UIAcceleration {
        #[deprecated = "UIAcceleration has been replaced by the CoreMotion framework"]
        #[method(timestamp)]
        pub unsafe fn timestamp(&self) -> NSTimeInterval;

        #[deprecated = "UIAcceleration has been replaced by the CoreMotion framework"]
        #[method(x)]
        pub unsafe fn x(&self) -> UIAccelerationValue;

        #[deprecated = "UIAcceleration has been replaced by the CoreMotion framework"]
        #[method(y)]
        pub unsafe fn y(&self) -> UIAccelerationValue;

        #[deprecated = "UIAcceleration has been replaced by the CoreMotion framework"]
        #[method(z)]
        pub unsafe fn z(&self) -> UIAccelerationValue;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIAcceleration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "UIAccelerometer has been replaced by the CoreMotion framework"]
    pub struct UIAccelerometer;

    unsafe impl ClassType for UIAccelerometer {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UIAccelerometer {}

extern_methods!(
    unsafe impl UIAccelerometer {
        #[deprecated = "UIAccelerometer has been replaced by the CoreMotion framework"]
        #[method_id(@__retain_semantics Other sharedAccelerometer)]
        pub unsafe fn sharedAccelerometer(mtm: MainThreadMarker) -> Retained<UIAccelerometer>;

        #[deprecated = "UIAccelerometer has been replaced by the CoreMotion framework"]
        #[method(updateInterval)]
        pub unsafe fn updateInterval(&self) -> NSTimeInterval;

        #[deprecated = "UIAccelerometer has been replaced by the CoreMotion framework"]
        #[method(setUpdateInterval:)]
        pub unsafe fn setUpdateInterval(&self, update_interval: NSTimeInterval);

        #[deprecated = "UIAccelerometer has been replaced by the CoreMotion framework"]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIAccelerometerDelegate>>>;

        #[deprecated = "UIAccelerometer has been replaced by the CoreMotion framework"]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UIAccelerometerDelegate>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIAccelerometer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    #[deprecated = "UIAcceleration has been replaced by the CoreMotion framework"]
    pub unsafe trait UIAccelerometerDelegate: NSObjectProtocol + MainThreadOnly {
        #[deprecated]
        #[optional]
        #[method(accelerometer:didAccelerate:)]
        unsafe fn accelerometer_didAccelerate(
            &self,
            accelerometer: &UIAccelerometer,
            acceleration: &UIAcceleration,
        );
    }

    unsafe impl ProtocolType for dyn UIAccelerometerDelegate {}
);

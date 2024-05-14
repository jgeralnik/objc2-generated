//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CMSensorDataList;

    unsafe impl ClassType for CMSensorDataList {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSFastEnumeration for CMSensorDataList {}

unsafe impl NSObjectProtocol for CMSensorDataList {}

extern_methods!(
    unsafe impl CMSensorDataList {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CMSensorDataList {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CMSensorRecorder;

    unsafe impl ClassType for CMSensorRecorder {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for CMSensorRecorder {}

extern_methods!(
    unsafe impl CMSensorRecorder {
        #[method(isAccelerometerRecordingAvailable)]
        pub unsafe fn isAccelerometerRecordingAvailable() -> bool;

        #[cfg(feature = "CMAuthorization")]
        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus() -> CMAuthorizationStatus;

        #[deprecated]
        #[method(isAuthorizedForRecording)]
        pub unsafe fn isAuthorizedForRecording() -> bool;

        #[method_id(@__retain_semantics Other accelerometerDataFromDate:toDate:)]
        pub unsafe fn accelerometerDataFromDate_toDate(
            &self,
            from_date: &NSDate,
            to_date: &NSDate,
        ) -> Option<Id<CMSensorDataList>>;

        #[method(recordAccelerometerForDuration:)]
        pub unsafe fn recordAccelerometerForDuration(&self, duration: NSTimeInterval);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CMSensorRecorder {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

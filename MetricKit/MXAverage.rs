//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MXAverage<UnitType: ?Sized = AnyObject> {
        __superclass: NSObject,
        _inner0: PhantomData<*mut UnitType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<UnitType: ?Sized + Message> ClassType for MXAverage<UnitType> {
        type Super = NSObject;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }
    }
);

unsafe impl<UnitType: ?Sized + NSCoding> NSCoding for MXAverage<UnitType> {}

unsafe impl<UnitType: ?Sized> NSObjectProtocol for MXAverage<UnitType> {}

unsafe impl<UnitType: ?Sized + NSSecureCoding> NSSecureCoding for MXAverage<UnitType> {}

extern_methods!(
    unsafe impl<UnitType: Message> MXAverage<UnitType> {
        #[method_id(@__retain_semantics Other averageMeasurement)]
        pub unsafe fn averageMeasurement(&self) -> Retained<NSMeasurement<UnitType>>;

        #[method(sampleCount)]
        pub unsafe fn sampleCount(&self) -> NSInteger;

        #[method(standardDeviation)]
        pub unsafe fn standardDeviation(&self) -> c_double;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<UnitType: Message> MXAverage<UnitType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

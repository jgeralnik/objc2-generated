//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CMWaterSubmersionState(pub NSInteger);
impl CMWaterSubmersionState {
    #[doc(alias = "CMWaterSubmersionStateUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "CMWaterSubmersionStateNotSubmerged")]
    pub const NotSubmerged: Self = Self(1);
    #[doc(alias = "CMWaterSubmersionStateSubmerged")]
    pub const Submerged: Self = Self(2);
}

unsafe impl Encode for CMWaterSubmersionState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CMWaterSubmersionState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CMWaterSubmersionDepthState(pub NSInteger);
impl CMWaterSubmersionDepthState {
    #[doc(alias = "CMWaterSubmersionDepthStateUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "CMWaterSubmersionDepthStateNotSubmerged")]
    pub const NotSubmerged: Self = Self(100);
    #[doc(alias = "CMWaterSubmersionDepthStateSubmergedShallow")]
    pub const SubmergedShallow: Self = Self(200);
    #[doc(alias = "CMWaterSubmersionDepthStateSubmergedDeep")]
    pub const SubmergedDeep: Self = Self(300);
    #[doc(alias = "CMWaterSubmersionDepthStateApproachingMaxDepth")]
    pub const ApproachingMaxDepth: Self = Self(400);
    #[doc(alias = "CMWaterSubmersionDepthStatePastMaxDepth")]
    pub const PastMaxDepth: Self = Self(500);
    #[doc(alias = "CMWaterSubmersionDepthStateSensorDepthError")]
    pub const SensorDepthError: Self = Self(600);
}

unsafe impl Encode for CMWaterSubmersionDepthState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CMWaterSubmersionDepthState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CMWaterSubmersionEvent;

    unsafe impl ClassType for CMWaterSubmersionEvent {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CMWaterSubmersionEvent {}

unsafe impl NSCopying for CMWaterSubmersionEvent {}

unsafe impl CopyingHelper for CMWaterSubmersionEvent {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CMWaterSubmersionEvent {}

unsafe impl NSSecureCoding for CMWaterSubmersionEvent {}

extern_methods!(
    unsafe impl CMWaterSubmersionEvent {
        #[method_id(@__retain_semantics Other date)]
        pub unsafe fn date(&self) -> Retained<NSDate>;

        #[method(state)]
        pub unsafe fn state(&self) -> CMWaterSubmersionState;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CMWaterSubmersionEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CMWaterSubmersionMeasurement;

    unsafe impl ClassType for CMWaterSubmersionMeasurement {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CMWaterSubmersionMeasurement {}

unsafe impl NSCopying for CMWaterSubmersionMeasurement {}

unsafe impl CopyingHelper for CMWaterSubmersionMeasurement {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CMWaterSubmersionMeasurement {}

unsafe impl NSSecureCoding for CMWaterSubmersionMeasurement {}

extern_methods!(
    unsafe impl CMWaterSubmersionMeasurement {
        #[method_id(@__retain_semantics Other date)]
        pub unsafe fn date(&self) -> Retained<NSDate>;

        #[method_id(@__retain_semantics Other depth)]
        pub unsafe fn depth(&self) -> Option<Retained<NSMeasurement<NSUnitLength>>>;

        #[method_id(@__retain_semantics Other pressure)]
        pub unsafe fn pressure(&self) -> Option<Retained<NSMeasurement<NSUnitPressure>>>;

        #[method_id(@__retain_semantics Other surfacePressure)]
        pub unsafe fn surfacePressure(&self) -> Retained<NSMeasurement<NSUnitPressure>>;

        #[method(submersionState)]
        pub unsafe fn submersionState(&self) -> CMWaterSubmersionDepthState;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CMWaterSubmersionMeasurement {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CMWaterTemperature;

    unsafe impl ClassType for CMWaterTemperature {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CMWaterTemperature {}

unsafe impl NSCopying for CMWaterTemperature {}

unsafe impl CopyingHelper for CMWaterTemperature {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CMWaterTemperature {}

unsafe impl NSSecureCoding for CMWaterTemperature {}

extern_methods!(
    unsafe impl CMWaterTemperature {
        #[method_id(@__retain_semantics Other date)]
        pub unsafe fn date(&self) -> Retained<NSDate>;

        #[method_id(@__retain_semantics Other temperature)]
        pub unsafe fn temperature(&self) -> Retained<NSMeasurement<NSUnitTemperature>>;

        #[method_id(@__retain_semantics Other temperatureUncertainty)]
        pub unsafe fn temperatureUncertainty(&self) -> Retained<NSMeasurement<NSUnitTemperature>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CMWaterTemperature {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

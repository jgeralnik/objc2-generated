//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corelocation/clbeaconmajorvalue?language=objc)
pub type CLBeaconMajorValue = u16;

/// [Apple's documentation](https://developer.apple.com/documentation/corelocation/clbeaconminorvalue?language=objc)
pub type CLBeaconMinorValue = u16;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corelocation/clbeaconidentitycondition?language=objc)
    #[unsafe(super(CLCondition, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CLCondition")]
    pub struct CLBeaconIdentityCondition;
);

#[cfg(feature = "CLCondition")]
unsafe impl NSCoding for CLBeaconIdentityCondition {}

#[cfg(feature = "CLCondition")]
unsafe impl NSCopying for CLBeaconIdentityCondition {}

#[cfg(feature = "CLCondition")]
unsafe impl CopyingHelper for CLBeaconIdentityCondition {
    type Result = Self;
}

#[cfg(feature = "CLCondition")]
unsafe impl NSObjectProtocol for CLBeaconIdentityCondition {}

#[cfg(feature = "CLCondition")]
unsafe impl NSSecureCoding for CLBeaconIdentityCondition {}

extern_methods!(
    #[cfg(feature = "CLCondition")]
    unsafe impl CLBeaconIdentityCondition {
        #[method_id(@__retain_semantics Other UUID)]
        pub unsafe fn UUID(&self) -> Retained<NSUUID>;

        #[method_id(@__retain_semantics Other major)]
        pub unsafe fn major(&self) -> Option<Retained<NSNumber>>;

        #[method_id(@__retain_semantics Other minor)]
        pub unsafe fn minor(&self) -> Option<Retained<NSNumber>>;

        #[method_id(@__retain_semantics Init initWithUUID:)]
        pub unsafe fn initWithUUID(this: Allocated<Self>, uuid: &NSUUID) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithUUID:major:)]
        pub unsafe fn initWithUUID_major(
            this: Allocated<Self>,
            uuid: &NSUUID,
            major: CLBeaconMajorValue,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithUUID:major:minor:)]
        pub unsafe fn initWithUUID_major_minor(
            this: Allocated<Self>,
            uuid: &NSUUID,
            major: CLBeaconMajorValue,
            minor: CLBeaconMinorValue,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CLCondition`
    #[cfg(feature = "CLCondition")]
    unsafe impl CLBeaconIdentityCondition {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

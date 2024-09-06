//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "CLBeaconIdentityCondition", feature = "CLCondition"))]
    #[deprecated]
    pub struct CLBeaconIdentityConstraint;

    #[cfg(all(feature = "CLBeaconIdentityCondition", feature = "CLCondition"))]
    unsafe impl ClassType for CLBeaconIdentityConstraint {
        #[inherits(CLCondition, NSObject)]
        type Super = CLBeaconIdentityCondition;
    }
);

#[cfg(all(feature = "CLBeaconIdentityCondition", feature = "CLCondition"))]
unsafe impl NSCoding for CLBeaconIdentityConstraint {}

#[cfg(all(feature = "CLBeaconIdentityCondition", feature = "CLCondition"))]
unsafe impl NSCopying for CLBeaconIdentityConstraint {}

#[cfg(all(feature = "CLBeaconIdentityCondition", feature = "CLCondition"))]
unsafe impl CopyingHelper for CLBeaconIdentityConstraint {
    type Result = Self;
}

#[cfg(all(feature = "CLBeaconIdentityCondition", feature = "CLCondition"))]
unsafe impl NSObjectProtocol for CLBeaconIdentityConstraint {}

#[cfg(all(feature = "CLBeaconIdentityCondition", feature = "CLCondition"))]
unsafe impl NSSecureCoding for CLBeaconIdentityConstraint {}

extern_methods!(
    #[cfg(all(feature = "CLBeaconIdentityCondition", feature = "CLCondition"))]
    unsafe impl CLBeaconIdentityConstraint {}
);

extern_methods!(
    /// Methods declared on superclass `CLBeaconIdentityCondition`
    #[cfg(all(feature = "CLBeaconIdentityCondition", feature = "CLCondition"))]
    unsafe impl CLBeaconIdentityConstraint {
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
    #[cfg(all(feature = "CLBeaconIdentityCondition", feature = "CLCondition"))]
    unsafe impl CLBeaconIdentityConstraint {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

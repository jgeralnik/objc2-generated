//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CLRegion")]
    #[deprecated]
    pub struct CLBeaconRegion;

    #[cfg(feature = "CLRegion")]
    unsafe impl ClassType for CLBeaconRegion {
        #[inherits(NSObject)]
        type Super = CLRegion;
    }
);

#[cfg(feature = "CLRegion")]
unsafe impl NSCoding for CLBeaconRegion {}

#[cfg(feature = "CLRegion")]
unsafe impl NSCopying for CLBeaconRegion {}

#[cfg(feature = "CLRegion")]
unsafe impl CopyingHelper for CLBeaconRegion {
    type Result = Self;
}

#[cfg(feature = "CLRegion")]
unsafe impl NSObjectProtocol for CLBeaconRegion {}

#[cfg(feature = "CLRegion")]
unsafe impl NSSecureCoding for CLBeaconRegion {}

extern_methods!(
    #[cfg(feature = "CLRegion")]
    unsafe impl CLBeaconRegion {
        #[method_id(@__retain_semantics Init initWithUUID:identifier:)]
        pub unsafe fn initWithUUID_identifier(
            this: Allocated<Self>,
            uuid: &NSUUID,
            identifier: &NSString,
        ) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init initWithProximityUUID:identifier:)]
        pub unsafe fn initWithProximityUUID_identifier(
            this: Allocated<Self>,
            proximity_uuid: &NSUUID,
            identifier: &NSString,
        ) -> Retained<Self>;

        #[cfg(feature = "CLBeaconIdentityCondition")]
        #[method_id(@__retain_semantics Init initWithUUID:major:identifier:)]
        pub unsafe fn initWithUUID_major_identifier(
            this: Allocated<Self>,
            uuid: &NSUUID,
            major: CLBeaconMajorValue,
            identifier: &NSString,
        ) -> Retained<Self>;

        #[cfg(feature = "CLBeaconIdentityCondition")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithProximityUUID:major:identifier:)]
        pub unsafe fn initWithProximityUUID_major_identifier(
            this: Allocated<Self>,
            proximity_uuid: &NSUUID,
            major: CLBeaconMajorValue,
            identifier: &NSString,
        ) -> Retained<Self>;

        #[cfg(feature = "CLBeaconIdentityCondition")]
        #[method_id(@__retain_semantics Init initWithUUID:major:minor:identifier:)]
        pub unsafe fn initWithUUID_major_minor_identifier(
            this: Allocated<Self>,
            uuid: &NSUUID,
            major: CLBeaconMajorValue,
            minor: CLBeaconMinorValue,
            identifier: &NSString,
        ) -> Retained<Self>;

        #[cfg(feature = "CLBeaconIdentityCondition")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithProximityUUID:major:minor:identifier:)]
        pub unsafe fn initWithProximityUUID_major_minor_identifier(
            this: Allocated<Self>,
            proximity_uuid: &NSUUID,
            major: CLBeaconMajorValue,
            minor: CLBeaconMinorValue,
            identifier: &NSString,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "CLBeaconIdentityCondition",
            feature = "CLBeaconIdentityConstraint",
            feature = "CLCondition"
        ))]
        #[method_id(@__retain_semantics Init initWithBeaconIdentityConstraint:identifier:)]
        pub unsafe fn initWithBeaconIdentityConstraint_identifier(
            this: Allocated<Self>,
            beacon_identity_constraint: &CLBeaconIdentityConstraint,
            identifier: &NSString,
        ) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other peripheralDataWithMeasuredPower:)]
        pub unsafe fn peripheralDataWithMeasuredPower(
            &self,
            measured_power: Option<&NSNumber>,
        ) -> Retained<NSMutableDictionary<NSString, AnyObject>>;

        #[cfg(all(
            feature = "CLBeaconIdentityCondition",
            feature = "CLBeaconIdentityConstraint",
            feature = "CLCondition"
        ))]
        #[method_id(@__retain_semantics Other beaconIdentityConstraint)]
        pub unsafe fn beaconIdentityConstraint(&self) -> Retained<CLBeaconIdentityConstraint>;

        #[method_id(@__retain_semantics Other UUID)]
        pub unsafe fn UUID(&self) -> Retained<NSUUID>;

        #[deprecated]
        #[method_id(@__retain_semantics Other proximityUUID)]
        pub unsafe fn proximityUUID(&self) -> Retained<NSUUID>;

        #[deprecated]
        #[method_id(@__retain_semantics Other major)]
        pub unsafe fn major(&self) -> Option<Retained<NSNumber>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other minor)]
        pub unsafe fn minor(&self) -> Option<Retained<NSNumber>>;

        #[deprecated]
        #[method(notifyEntryStateOnDisplay)]
        pub unsafe fn notifyEntryStateOnDisplay(&self) -> bool;

        #[deprecated]
        #[method(setNotifyEntryStateOnDisplay:)]
        pub unsafe fn setNotifyEntryStateOnDisplay(&self, notify_entry_state_on_display: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `CLRegion`
    #[cfg(feature = "CLRegion")]
    unsafe impl CLBeaconRegion {
        #[cfg(feature = "CLLocation")]
        #[deprecated = "Please see CLCircularRegion"]
        #[method_id(@__retain_semantics Init initCircularRegionWithCenter:radius:identifier:)]
        pub unsafe fn initCircularRegionWithCenter_radius_identifier(
            this: Allocated<Self>,
            center: CLLocationCoordinate2D,
            radius: CLLocationDistance,
            identifier: &NSString,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CLRegion")]
    unsafe impl CLBeaconRegion {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLBeacon;

    unsafe impl ClassType for CLBeacon {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for CLBeacon {}

unsafe impl NSCopying for CLBeacon {}

unsafe impl CopyingHelper for CLBeacon {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CLBeacon {}

unsafe impl NSSecureCoding for CLBeacon {}

extern_methods!(
    unsafe impl CLBeacon {
        #[method_id(@__retain_semantics Other timestamp)]
        pub unsafe fn timestamp(&self) -> Retained<NSDate>;

        #[method_id(@__retain_semantics Other UUID)]
        pub unsafe fn UUID(&self) -> Retained<NSUUID>;

        #[deprecated]
        #[method_id(@__retain_semantics Other proximityUUID)]
        pub unsafe fn proximityUUID(&self) -> Retained<NSUUID>;

        #[method_id(@__retain_semantics Other major)]
        pub unsafe fn major(&self) -> Retained<NSNumber>;

        #[method_id(@__retain_semantics Other minor)]
        pub unsafe fn minor(&self) -> Retained<NSNumber>;

        #[cfg(feature = "CLRegion")]
        #[method(proximity)]
        pub unsafe fn proximity(&self) -> CLProximity;

        #[cfg(feature = "CLLocation")]
        #[method(accuracy)]
        pub unsafe fn accuracy(&self) -> CLLocationAccuracy;

        #[method(rssi)]
        pub unsafe fn rssi(&self) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CLBeacon {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

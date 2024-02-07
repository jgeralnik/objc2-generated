//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(c_int)]
    pub enum CLDeviceOrientation {
        CLDeviceOrientationUnknown = 0,
        CLDeviceOrientationPortrait = 1,
        CLDeviceOrientationPortraitUpsideDown = 2,
        CLDeviceOrientationLandscapeLeft = 3,
        CLDeviceOrientationLandscapeRight = 4,
        CLDeviceOrientationFaceUp = 5,
        CLDeviceOrientationFaceDown = 6,
    }
);

ns_enum!(
    #[underlying(c_int)]
    pub enum CLAuthorizationStatus {
        kCLAuthorizationStatusNotDetermined = 0,
        kCLAuthorizationStatusRestricted = 1,
        kCLAuthorizationStatusDenied = 2,
        kCLAuthorizationStatusAuthorizedAlways = 3,
        kCLAuthorizationStatusAuthorizedWhenInUse = 4,
        #[deprecated = "Use kCLAuthorizationStatusAuthorizedAlways"]
        kCLAuthorizationStatusAuthorized =
            CLAuthorizationStatus::kCLAuthorizationStatusAuthorizedAlways.0,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CLAccuracyAuthorization {
        CLAccuracyAuthorizationFullAccuracy = 0,
        CLAccuracyAuthorizationReducedAccuracy = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CLActivityType {
        CLActivityTypeOther = 1,
        CLActivityTypeAutomotiveNavigation = 2,
        CLActivityTypeFitness = 3,
        CLActivityTypeOtherNavigation = 4,
        CLActivityTypeAirborne = 5,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreLocation_CLLocationManager")]
    pub struct CLLocationManager;

    #[cfg(feature = "CoreLocation_CLLocationManager")]
    unsafe impl ClassType for CLLocationManager {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreLocation_CLLocationManager")]
unsafe impl NSObjectProtocol for CLLocationManager {}

extern_methods!(
    #[cfg(feature = "CoreLocation_CLLocationManager")]
    unsafe impl CLLocationManager {
        #[method(locationServicesEnabled)]
        pub unsafe fn locationServicesEnabled_class() -> bool;

        #[method(headingAvailable)]
        pub unsafe fn headingAvailable_class() -> bool;

        #[method(significantLocationChangeMonitoringAvailable)]
        pub unsafe fn significantLocationChangeMonitoringAvailable() -> bool;

        #[method(isMonitoringAvailableForClass:)]
        pub unsafe fn isMonitoringAvailableForClass(region_class: &AnyClass) -> bool;

        #[deprecated]
        #[method(regionMonitoringAvailable)]
        pub unsafe fn regionMonitoringAvailable() -> bool;

        #[deprecated = "Use +isMonitoringAvailableForClass: and -authorizationStatus instead"]
        #[method(regionMonitoringEnabled)]
        pub unsafe fn regionMonitoringEnabled() -> bool;

        #[method(isRangingAvailable)]
        pub unsafe fn isRangingAvailable() -> bool;

        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus(&self) -> CLAuthorizationStatus;

        #[deprecated]
        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus_class() -> CLAuthorizationStatus;

        #[method(accuracyAuthorization)]
        pub unsafe fn accuracyAuthorization(&self) -> CLAccuracyAuthorization;

        #[method(isAuthorizedForWidgetUpdates)]
        pub unsafe fn isAuthorizedForWidgetUpdates(&self) -> bool;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn CLLocationManagerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn CLLocationManagerDelegate>>,
        );

        #[deprecated]
        #[method(locationServicesEnabled)]
        pub unsafe fn locationServicesEnabled(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Set the purpose string in Info.plist using key NSLocationUsageDescription"]
        #[method_id(@__retain_semantics Other purpose)]
        pub unsafe fn purpose(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Set the purpose string in Info.plist using key NSLocationUsageDescription"]
        #[method(setPurpose:)]
        pub unsafe fn setPurpose(&self, purpose: Option<&NSString>);

        #[method(activityType)]
        pub unsafe fn activityType(&self) -> CLActivityType;

        #[method(setActivityType:)]
        pub unsafe fn setActivityType(&self, activity_type: CLActivityType);

        #[method(distanceFilter)]
        pub unsafe fn distanceFilter(&self) -> CLLocationDistance;

        #[method(setDistanceFilter:)]
        pub unsafe fn setDistanceFilter(&self, distance_filter: CLLocationDistance);

        #[method(desiredAccuracy)]
        pub unsafe fn desiredAccuracy(&self) -> CLLocationAccuracy;

        #[method(setDesiredAccuracy:)]
        pub unsafe fn setDesiredAccuracy(&self, desired_accuracy: CLLocationAccuracy);

        #[method(pausesLocationUpdatesAutomatically)]
        pub unsafe fn pausesLocationUpdatesAutomatically(&self) -> bool;

        #[method(setPausesLocationUpdatesAutomatically:)]
        pub unsafe fn setPausesLocationUpdatesAutomatically(
            &self,
            pauses_location_updates_automatically: bool,
        );

        #[method(allowsBackgroundLocationUpdates)]
        pub unsafe fn allowsBackgroundLocationUpdates(&self) -> bool;

        #[method(setAllowsBackgroundLocationUpdates:)]
        pub unsafe fn setAllowsBackgroundLocationUpdates(
            &self,
            allows_background_location_updates: bool,
        );

        #[method(showsBackgroundLocationIndicator)]
        pub unsafe fn showsBackgroundLocationIndicator(&self) -> bool;

        #[method(setShowsBackgroundLocationIndicator:)]
        pub unsafe fn setShowsBackgroundLocationIndicator(
            &self,
            shows_background_location_indicator: bool,
        );

        #[cfg(feature = "CoreLocation_CLLocation")]
        #[method_id(@__retain_semantics Other location)]
        pub unsafe fn location(&self) -> Option<Id<CLLocation>>;

        #[deprecated]
        #[method(headingAvailable)]
        pub unsafe fn headingAvailable(&self) -> bool;

        #[method(headingFilter)]
        pub unsafe fn headingFilter(&self) -> CLLocationDegrees;

        #[method(setHeadingFilter:)]
        pub unsafe fn setHeadingFilter(&self, heading_filter: CLLocationDegrees);

        #[method(headingOrientation)]
        pub unsafe fn headingOrientation(&self) -> CLDeviceOrientation;

        #[method(setHeadingOrientation:)]
        pub unsafe fn setHeadingOrientation(&self, heading_orientation: CLDeviceOrientation);

        #[cfg(feature = "CoreLocation_CLHeading")]
        #[method_id(@__retain_semantics Other heading)]
        pub unsafe fn heading(&self) -> Option<Id<CLHeading>>;

        #[method(maximumRegionMonitoringDistance)]
        pub unsafe fn maximumRegionMonitoringDistance(&self) -> CLLocationDistance;

        #[cfg(all(feature = "CoreLocation_CLRegion", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Other monitoredRegions)]
        pub unsafe fn monitoredRegions(&self) -> Id<NSSet<CLRegion>>;

        #[cfg(all(feature = "CoreLocation_CLRegion", feature = "Foundation_NSSet"))]
        #[deprecated = "Use -rangedBeaconConstraints"]
        #[method_id(@__retain_semantics Other rangedRegions)]
        pub unsafe fn rangedRegions(&self) -> Id<NSSet<CLRegion>>;

        #[cfg(all(
            feature = "CoreLocation_CLBeaconIdentityConstraint",
            feature = "Foundation_NSSet"
        ))]
        #[method_id(@__retain_semantics Other rangedBeaconConstraints)]
        pub unsafe fn rangedBeaconConstraints(&self) -> Id<NSSet<CLBeaconIdentityConstraint>>;

        #[method(requestWhenInUseAuthorization)]
        pub unsafe fn requestWhenInUseAuthorization(&self);

        #[method(requestAlwaysAuthorization)]
        pub unsafe fn requestAlwaysAuthorization(&self);

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(requestTemporaryFullAccuracyAuthorizationWithPurposeKey:completion:)]
        pub unsafe fn requestTemporaryFullAccuracyAuthorizationWithPurposeKey_completion(
            &self,
            purpose_key: &NSString,
            completion: Option<&Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(requestTemporaryFullAccuracyAuthorizationWithPurposeKey:)]
        pub unsafe fn requestTemporaryFullAccuracyAuthorizationWithPurposeKey(
            &self,
            purpose_key: &NSString,
        );

        #[method(startUpdatingLocation)]
        pub unsafe fn startUpdatingLocation(&self);

        #[method(stopUpdatingLocation)]
        pub unsafe fn stopUpdatingLocation(&self);

        #[method(requestLocation)]
        pub unsafe fn requestLocation(&self);

        #[method(startUpdatingHeading)]
        pub unsafe fn startUpdatingHeading(&self);

        #[method(stopUpdatingHeading)]
        pub unsafe fn stopUpdatingHeading(&self);

        #[method(dismissHeadingCalibrationDisplay)]
        pub unsafe fn dismissHeadingCalibrationDisplay(&self);

        #[method(startMonitoringSignificantLocationChanges)]
        pub unsafe fn startMonitoringSignificantLocationChanges(&self);

        #[method(stopMonitoringSignificantLocationChanges)]
        pub unsafe fn stopMonitoringSignificantLocationChanges(&self);

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method(startMonitoringLocationPushesWithCompletion:)]
        pub unsafe fn startMonitoringLocationPushesWithCompletion(
            &self,
            completion: Option<&Block<dyn Fn(*mut NSData, *mut NSError)>>,
        );

        #[method(stopMonitoringLocationPushes)]
        pub unsafe fn stopMonitoringLocationPushes(&self);

        #[cfg(feature = "CoreLocation_CLRegion")]
        #[deprecated]
        #[method(startMonitoringForRegion:desiredAccuracy:)]
        pub unsafe fn startMonitoringForRegion_desiredAccuracy(
            &self,
            region: &CLRegion,
            accuracy: CLLocationAccuracy,
        );

        #[cfg(feature = "CoreLocation_CLRegion")]
        #[deprecated]
        #[method(stopMonitoringForRegion:)]
        pub unsafe fn stopMonitoringForRegion(&self, region: &CLRegion);

        #[cfg(feature = "CoreLocation_CLRegion")]
        #[deprecated]
        #[method(startMonitoringForRegion:)]
        pub unsafe fn startMonitoringForRegion(&self, region: &CLRegion);

        #[cfg(feature = "CoreLocation_CLRegion")]
        #[deprecated]
        #[method(requestStateForRegion:)]
        pub unsafe fn requestStateForRegion(&self, region: &CLRegion);

        #[cfg(feature = "CoreLocation_CLBeaconRegion")]
        #[deprecated = "Use -startRangingBeaconsSatisfyingConstraint:"]
        #[method(startRangingBeaconsInRegion:)]
        pub unsafe fn startRangingBeaconsInRegion(&self, region: &CLBeaconRegion);

        #[cfg(feature = "CoreLocation_CLBeaconRegion")]
        #[deprecated = "Use -stopRangingBeaconsSatisfyingConstraint:"]
        #[method(stopRangingBeaconsInRegion:)]
        pub unsafe fn stopRangingBeaconsInRegion(&self, region: &CLBeaconRegion);

        #[cfg(feature = "CoreLocation_CLBeaconIdentityConstraint")]
        #[method(startRangingBeaconsSatisfyingConstraint:)]
        pub unsafe fn startRangingBeaconsSatisfyingConstraint(
            &self,
            constraint: &CLBeaconIdentityConstraint,
        );

        #[cfg(feature = "CoreLocation_CLBeaconIdentityConstraint")]
        #[method(stopRangingBeaconsSatisfyingConstraint:)]
        pub unsafe fn stopRangingBeaconsSatisfyingConstraint(
            &self,
            constraint: &CLBeaconIdentityConstraint,
        );

        #[deprecated = "You can remove calls to this method"]
        #[method(allowDeferredLocationUpdatesUntilTraveled:timeout:)]
        pub unsafe fn allowDeferredLocationUpdatesUntilTraveled_timeout(
            &self,
            distance: CLLocationDistance,
            timeout: NSTimeInterval,
        );

        #[deprecated = "You can remove calls to this method"]
        #[method(disallowDeferredLocationUpdates)]
        pub unsafe fn disallowDeferredLocationUpdates(&self);

        #[deprecated = "You can remove calls to this method"]
        #[method(deferredLocationUpdatesAvailable)]
        pub unsafe fn deferredLocationUpdatesAvailable() -> bool;

        #[cfg(all(
            feature = "CoreLocation_CLLocation",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(requestHistoricalLocationsWithPurposeKey:sampleCount:completionHandler:)]
        pub unsafe fn requestHistoricalLocationsWithPurposeKey_sampleCount_completionHandler(
            &self,
            purpose_key: &NSString,
            sample_count: NSInteger,
            handler: &Block<dyn Fn(NonNull<NSArray<CLLocation>>, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreLocation_CLLocationManager")]
    unsafe impl CLLocationManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

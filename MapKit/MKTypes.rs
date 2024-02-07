//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MKMapType {
        #[doc(alias = "MKMapTypeStandard")]
        Standard = 0,
        #[doc(alias = "MKMapTypeSatellite")]
        Satellite = 1,
        #[doc(alias = "MKMapTypeHybrid")]
        Hybrid = 2,
        #[doc(alias = "MKMapTypeSatelliteFlyover")]
        SatelliteFlyover = 3,
        #[doc(alias = "MKMapTypeHybridFlyover")]
        HybridFlyover = 4,
        #[doc(alias = "MKMapTypeMutedStandard")]
        MutedStandard = 5,
    }
);

extern_static!(MKErrorDomain: &'static NSString);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MKErrorCode {
        MKErrorUnknown = 1,
        MKErrorServerFailure = 2,
        MKErrorLoadingThrottled = 3,
        MKErrorPlacemarkNotFound = 4,
        MKErrorDirectionsNotFound = 5,
        MKErrorDecodingFailed = 6,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MKFeatureVisibility {
        #[doc(alias = "MKFeatureVisibilityAdaptive")]
        Adaptive = 0,
        #[doc(alias = "MKFeatureVisibilityHidden")]
        Hidden = 1,
        #[doc(alias = "MKFeatureVisibilityVisible")]
        Visible = 2,
    }
);

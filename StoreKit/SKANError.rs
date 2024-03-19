//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static SKANErrorDomain: &'static NSString;
}

ns_error_enum!(
    #[underlying(NSInteger)]
    pub enum SKANError {
        #[doc(alias = "SKANErrorImpressionMissingRequiredValue")]
        ImpressionMissingRequiredValue = 0,
        #[doc(alias = "SKANErrorUnsupported")]
        Unsupported = 1,
        #[doc(alias = "SKANErrorAdNetworkIdMissing")]
        AdNetworkIdMissing = 2,
        #[doc(alias = "SKANErrorMismatchedSourceAppId")]
        MismatchedSourceAppId = 3,
        #[doc(alias = "SKANErrorImpressionNotFound")]
        ImpressionNotFound = 4,
        #[doc(alias = "SKANErrorInvalidCampaignId")]
        InvalidCampaignId = 5,
        #[doc(alias = "SKANErrorInvalidConversionValue")]
        InvalidConversionValue = 6,
        #[doc(alias = "SKANErrorInvalidSourceAppId")]
        InvalidSourceAppId = 7,
        #[doc(alias = "SKANErrorInvalidAdvertisedAppId")]
        InvalidAdvertisedAppId = 8,
        #[doc(alias = "SKANErrorInvalidVersion")]
        InvalidVersion = 9,
        #[doc(alias = "SKANErrorUnknown")]
        Unknown = 10,
        #[doc(alias = "SKANErrorImpressionTooShort")]
        ImpressionTooShort = 11,
    }
);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_static!(CLLocationPushServiceErrorDomain: Option<&'static NSErrorDomain>);

ns_error_enum!(
    #[underlying(NSInteger)]
    pub enum CLLocationPushServiceError {
        #[doc(alias = "CLLocationPushServiceErrorUnknown")]
        Unknown = 0,
        #[doc(alias = "CLLocationPushServiceErrorMissingPushExtension")]
        MissingPushExtension = 1,
        #[doc(alias = "CLLocationPushServiceErrorMissingPushServerEnvironment")]
        MissingPushServerEnvironment = 2,
        #[doc(alias = "CLLocationPushServiceErrorMissingEntitlement")]
        MissingEntitlement = 3,
        #[doc(alias = "CLLocationPushServiceErrorUnsupportedPlatform")]
        UnsupportedPlatform = 4,
    }
);

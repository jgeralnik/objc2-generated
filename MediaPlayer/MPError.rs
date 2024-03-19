//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static MPErrorDomain: &'static NSString;
}

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MPErrorCode {
        MPErrorUnknown = 0,
        MPErrorPermissionDenied = 1,
        MPErrorCloudServiceCapabilityMissing = 2,
        MPErrorNetworkConnectionFailed = 3,
        MPErrorNotFound = 4,
        MPErrorNotSupported = 5,
        MPErrorCancelled = 6,
        MPErrorRequestTimedOut = 7,
    }
);

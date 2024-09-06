//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SMAppServiceStatus(pub NSInteger);
impl SMAppServiceStatus {
    #[doc(alias = "SMAppServiceStatusNotRegistered")]
    pub const NotRegistered: Self = Self(0);
    #[doc(alias = "SMAppServiceStatusEnabled")]
    pub const Enabled: Self = Self(1);
    #[doc(alias = "SMAppServiceStatusRequiresApproval")]
    pub const RequiresApproval: Self = Self(2);
    #[doc(alias = "SMAppServiceStatusNotFound")]
    pub const NotFound: Self = Self(3);
}

unsafe impl Encode for SMAppServiceStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SMAppServiceStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SMAppService;

    unsafe impl ClassType for SMAppService {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for SMAppService {}

extern_methods!(
    unsafe impl SMAppService {
        #[method_id(@__retain_semantics Other loginItemServiceWithIdentifier:)]
        pub unsafe fn loginItemServiceWithIdentifier(identifier: &NSString) -> Retained<Self>;

        #[method_id(@__retain_semantics Other mainAppService)]
        pub unsafe fn mainAppService() -> Retained<SMAppService>;

        #[method_id(@__retain_semantics Other agentServiceWithPlistName:)]
        pub unsafe fn agentServiceWithPlistName(plist_name: &NSString) -> Retained<Self>;

        #[method_id(@__retain_semantics Other daemonServiceWithPlistName:)]
        pub unsafe fn daemonServiceWithPlistName(plist_name: &NSString) -> Retained<Self>;

        #[method(registerAndReturnError:_)]
        pub unsafe fn registerAndReturnError(&self) -> Result<(), Retained<NSError>>;

        #[method(unregisterAndReturnError:_)]
        pub unsafe fn unregisterAndReturnError(&self) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "block2")]
        #[method(unregisterWithCompletionHandler:)]
        pub unsafe fn unregisterWithCompletionHandler(
            &self,
            handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[method(status)]
        pub unsafe fn status(&self) -> SMAppServiceStatus;

        #[method(statusForLegacyURL:)]
        pub unsafe fn statusForLegacyURL(url: &NSURL) -> SMAppServiceStatus;

        #[method(openSystemSettingsLoginItems)]
        pub unsafe fn openSystemSettingsLoginItems();
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SMAppService {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

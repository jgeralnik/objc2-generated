//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNInstantMessageAddress;

    unsafe impl ClassType for CNInstantMessageAddress {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for CNInstantMessageAddress {}

unsafe impl NSCopying for CNInstantMessageAddress {}

unsafe impl CopyingHelper for CNInstantMessageAddress {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CNInstantMessageAddress {}

unsafe impl NSSecureCoding for CNInstantMessageAddress {}

extern_methods!(
    unsafe impl CNInstantMessageAddress {
        #[method_id(@__retain_semantics Other username)]
        pub unsafe fn username(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other service)]
        pub unsafe fn service(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Init initWithUsername:service:)]
        pub unsafe fn initWithUsername_service(
            this: Allocated<Self>,
            username: &NSString,
            service: &NSString,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other localizedStringForKey:)]
        pub unsafe fn localizedStringForKey(key: &NSString) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other localizedStringForService:)]
        pub unsafe fn localizedStringForService(service: &NSString) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNInstantMessageAddress {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    pub static CNInstantMessageAddressUsernameKey: &'static NSString;
}

extern "C" {
    pub static CNInstantMessageAddressServiceKey: &'static NSString;
}

extern "C" {
    pub static CNInstantMessageServiceAIM: &'static NSString;
}

extern "C" {
    pub static CNInstantMessageServiceFacebook: &'static NSString;
}

extern "C" {
    pub static CNInstantMessageServiceGaduGadu: &'static NSString;
}

extern "C" {
    pub static CNInstantMessageServiceGoogleTalk: &'static NSString;
}

extern "C" {
    pub static CNInstantMessageServiceICQ: &'static NSString;
}

extern "C" {
    pub static CNInstantMessageServiceJabber: &'static NSString;
}

extern "C" {
    pub static CNInstantMessageServiceMSN: &'static NSString;
}

extern "C" {
    pub static CNInstantMessageServiceQQ: &'static NSString;
}

extern "C" {
    pub static CNInstantMessageServiceSkype: &'static NSString;
}

extern "C" {
    pub static CNInstantMessageServiceYahoo: &'static NSString;
}
